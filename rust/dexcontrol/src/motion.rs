// Copyright (C) 2026 Dexmate Inc.
//
// This software is dual-licensed:
//
// 1. GNU Affero General Public License v3.0 (AGPL-3.0)
//    See LICENSE-AGPL for details
//
// 2. Commercial License
//    For commercial licensing terms, contact: contact@dexmate.ai
//! Multi-component commands and owned motion groups.
use crate::*;
use std::collections::BTreeMap;
#[derive(Debug, Clone)]
pub struct TrajectoryTrack {
    pub positions: Vec<Vec<f64>>,
    pub velocities: Option<Vec<Vec<f64>>>,
}
impl Robot {
    pub fn set_joint_positions(
        &self,
        targets: &BTreeMap<String, Vec<f64>>,
        options: CommandOptions,
    ) -> Result<()> {
        let names: Vec<CString> = targets.keys().map(|s| string(s)).collect::<Result<_>>()?;
        let names: Vec<_> = names.iter().map(|s| s.as_ptr()).collect();
        let positions: Vec<_> = targets.values().map(|v| v.as_ptr()).collect();
        let counts: Vec<_> = targets.values().map(Vec::len).collect();
        let options = options.native()?;
        call(|e| unsafe {
            ffi::dex_robot_set_joint_pos_multi(
                self.0.as_ptr(),
                names.as_ptr(),
                positions.as_ptr(),
                counts.as_ptr(),
                names.len(),
                &options,
                e,
            )
        })
    }
    pub fn move_to_joint_positions(
        &self,
        targets: &BTreeMap<String, Vec<f64>>,
        options: MotionOptions,
    ) -> Result<MotionGroup> {
        let names: Vec<CString> = targets.keys().map(|s| string(s)).collect::<Result<_>>()?;
        let names: Vec<_> = names.iter().map(|s| s.as_ptr()).collect();
        let positions: Vec<_> = targets.values().map(|v| v.as_ptr()).collect();
        let counts: Vec<_> = targets.values().map(Vec::len).collect();
        let options = options.native()?;
        MotionGroup::start(|out, e| unsafe {
            ffi::dex_robot_move_to_joint_pos_multi(
                self.0.as_ptr(),
                names.as_ptr(),
                positions.as_ptr(),
                counts.as_ptr(),
                names.len(),
                &options,
                out,
                e,
            )
        })
    }
    /// Best-effort submission with a per-component JSON report. A successful return
    /// means the report is available, not that every command was delivered.
    /// Inspect `all_delivered` in the report before proceeding to later stages.
    pub fn start_joint_motions(
        &self,
        targets: &BTreeMap<String, Vec<f64>>,
        options: MotionOptions,
    ) -> Result<MotionSubmission> {
        let strings: Vec<CString> = targets.keys().map(|s| string(s)).collect::<Result<_>>()?;
        let names: Vec<_> = strings.iter().map(|s| s.as_ptr()).collect();
        let positions: Vec<_> = targets.values().map(|v| v.as_ptr()).collect();
        let counts: Vec<_> = targets.values().map(Vec::len).collect();
        let options = options.native()?;
        let mut out = ptr::null_mut();
        let result = text(|json, e| unsafe {
            ffi::dex_robot_start_joint_motions_opt(
                self.0.as_ptr(),
                names.as_ptr(),
                positions.as_ptr(),
                counts.as_ptr(),
                names.len(),
                &options,
                &mut out,
                json,
                e,
            )
        });
        let group = NonNull::new(out).map(MotionGroup);
        let report_json = result?;
        let group = group.ok_or_else(|| invalid("Runtime returned no motion group"))?;
        Ok(MotionSubmission { group, report_json })
    }
    /// Streams the entire recording before returning; validation happens before publication.
    pub fn execute_trajectory(
        &self,
        tracks: &BTreeMap<String, TrajectoryTrack>,
        hz: f64,
        options: TrajectoryOptions,
    ) -> Result<()> {
        if tracks.is_empty() {
            return Err(invalid("At least one trajectory track is required"));
        }
        let ticks = tracks.values().next().unwrap().positions.len();
        let mut counts = Vec::new();
        let mut positions = Vec::new();
        let mut velocities = Vec::new();
        for track in tracks.values() {
            let joints = rectangular(&track.positions)?;
            if track.positions.len() != ticks {
                return Err(invalid(
                    "All trajectory tracks must have the same tick count",
                ));
            }
            if let Some(v) = &track.velocities {
                if rectangular(v)? != joints || v.len() != ticks {
                    return Err(invalid("Trajectory velocity shape differs from positions"));
                }
            }
            counts.push(joints);
            positions.push(
                track
                    .positions
                    .iter()
                    .flatten()
                    .copied()
                    .collect::<Vec<f64>>(),
            );
            velocities.push(
                track
                    .velocities
                    .as_ref()
                    .map(|v| v.iter().flatten().copied().collect::<Vec<f64>>()),
            );
        }
        let strings: Vec<CString> = tracks.keys().map(|s| string(s)).collect::<Result<_>>()?;
        let names: Vec<_> = strings.iter().map(|s| s.as_ptr()).collect();
        let positions: Vec<_> = positions.iter().map(|v| v.as_ptr()).collect();
        let velocities: Vec<_> = velocities
            .iter()
            .map(|v| v.as_ref().map_or(ptr::null(), |v| v.as_ptr()))
            .collect();
        let options = options.native()?;
        call(|e| unsafe {
            ffi::dex_robot_execute_trajectory_opt(
                self.0.as_ptr(),
                names.as_ptr(),
                positions.as_ptr(),
                velocities.as_ptr(),
                ticks,
                counts.as_ptr(),
                names.len(),
                hz,
                &options,
                e,
            )
        })
    }
}
impl MotionState {
    pub(crate) fn from_native(n: u32) -> Self {
        match n {
            0 => Self::Pending,
            1 => Self::Running,
            2 => Self::Succeeded,
            3 => Self::Cancelled,
            4 => Self::Failed,
            5 => Self::Superseded,
            n => Self::Unknown(n),
        }
    }
}
impl MotionHandle {
    pub fn id(&self) -> Result<u64> {
        let mut id = 0;
        call(|_| unsafe { ffi::dex_motion_id(self.0.as_ptr(), &mut id) })?;
        Ok(id)
    }
    pub fn message(&self) -> Result<String> {
        text(|out, e| unsafe { ffi::dex_motion_message(self.0.as_ptr(), out, e) })
    }
    pub fn refresh(&self) -> Result<MotionState> {
        let mut state = 0;
        call(|e| unsafe { ffi::dex_motion_refresh(self.0.as_ptr(), &mut state, e) })?;
        Ok(MotionState::from_native(state))
    }
    pub fn wait_with(&self, options: WaitOptions) -> Result<()> {
        let options = options.native()?;
        call(|e| unsafe { ffi::dex_motion_wait_success(self.0.as_ptr(), &options, e) })
    }
}
pub struct MotionGroup(NonNull<ffi::dex_motion_group_t>);
impl MotionGroup {
    fn start(
        f: impl FnOnce(*mut *mut ffi::dex_motion_group_t, *mut ffi::dex_error_t) -> i32,
    ) -> Result<Self> {
        let mut out = ptr::null_mut();
        let result = call(|e| f(&mut out, e));
        if let Err(e) = result {
            if !out.is_null() {
                unsafe { ffi::dex_motion_group_release(out) }
            }
            return Err(e);
        }
        Ok(Self(NonNull::new(out).ok_or_else(|| {
            invalid("Runtime returned no motion group")
        })?))
    }
    pub fn len(&self) -> Result<usize> {
        let mut n = 0;
        call(|_| unsafe { ffi::dex_motion_group_len(self.0.as_ptr(), &mut n) })?;
        Ok(n)
    }
    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.len()? == 0)
    }
    pub fn member(&self, index: usize) -> Result<MotionHandle> {
        MotionHandle::start(|out, e| unsafe {
            ffi::dex_motion_group_member(self.0.as_ptr(), index, out, e)
        })
    }
    pub fn state(&self) -> Result<MotionState> {
        let mut n = 0;
        call(|e| unsafe { ffi::dex_motion_group_state(self.0.as_ptr(), &mut n, e) })?;
        Ok(MotionState::from_native(n))
    }
    pub fn cancel(&self) -> Result<()> {
        call(|e| unsafe { ffi::dex_motion_group_cancel(self.0.as_ptr(), e) })
    }
    /// Requires successful completion of every member, not merely terminal states.
    pub fn wait(&self, timeout: Duration) -> Result<()> {
        self.wait_with(Wait::Timeout(timeout).into())
    }
    /// Wait with explicit tolerance, settling interval and feedback freshness.
    pub fn wait_with(&self, options: WaitOptions) -> Result<()> {
        let wait = options.native()?;
        let mut n = 0;
        call(|e| unsafe { ffi::dex_motion_group_wait(self.0.as_ptr(), &wait, &mut n, e) })?;
        Self::require_success(MotionState::from_native(n))
    }
    fn require_success(state: MotionState) -> Result<()> {
        if state != MotionState::Succeeded {
            return Err(Error {
                code: ffi::DEX_RUNTIME_ERROR,
                message: format!("Motion group did not succeed: {state:?}"),
            });
        }
        Ok(())
    }
}
impl Drop for MotionGroup {
    fn drop(&mut self) {
        unsafe { ffi::dex_motion_group_release(self.0.as_ptr()) }
    }
}
// SAFETY: C ABI group operations synchronize internally; ownership prevents release during calls.
unsafe impl Send for MotionGroup {}
unsafe impl Sync for MotionGroup {}

/// The group contains only successfully submitted motions, in report order.
#[must_use = "Inspect delivery outcomes before waiting or starting another stage"]
pub struct MotionSubmission {
    pub group: MotionGroup,
    pub report_json: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_wait_requires_success_not_just_a_terminal_state() {
        assert!(MotionGroup::require_success(MotionState::Succeeded).is_ok());
        for state in [
            MotionState::Pending,
            MotionState::Running,
            MotionState::Failed,
            MotionState::Cancelled,
            MotionState::Superseded,
            MotionState::Unknown(99),
        ] {
            assert!(MotionGroup::require_success(state).is_err());
        }
    }
}
