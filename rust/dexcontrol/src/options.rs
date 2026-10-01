// Copyright (C) 2026 Dexmate Inc.
//
// This software is dual-licensed:
//
// 1. GNU Affero General Public License v3.0 (AGPL-3.0)
//    See LICENSE-AGPL for details
//
// 2. Commercial License
//    For commercial licensing terms, contact: contact@dexmate.ai
//! Command options retain native safety defaults unless explicitly overridden.
use crate::*;

/// A configured limit, an explicit positive duration, or an expert opt-out.
#[derive(Clone, Copy, Debug, Default)]
pub enum DurationLimit {
    #[default]
    Configured,
    Maximum(Duration),
    Unlimited,
}
impl DurationLimit {
    pub(crate) fn native(self) -> Result<u64> {
        match self {
            Self::Configured => Ok(0),
            Self::Maximum(d) => millis(d),
            Self::Unlimited => Ok(u64::MAX),
        }
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub enum StepLimit {
    #[default]
    Configured,
    Maximum(f64),
    Unlimited,
}
impl StepLimit {
    pub(crate) fn native(self) -> Result<f64> {
        match self {
            Self::Configured => Ok(0.0),
            Self::Unlimited => Ok(-1.0),
            Self::Maximum(v) if v.is_finite() && v > 0.0 => Ok(v),
            _ => Err(invalid("Step limit must be finite and positive")),
        }
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct WaitOptions {
    pub policy: Wait,
    pub convergence_tolerance: Option<f64>,
    pub settle_time: Duration,
    pub max_state_age: DurationLimit,
    pub wait_ceiling: DurationLimit,
}
impl From<Wait> for WaitOptions {
    fn from(policy: Wait) -> Self {
        Self {
            policy,
            ..Default::default()
        }
    }
}
impl WaitOptions {
    pub(crate) fn native(self) -> Result<ffi::dex_wait_policy_t> {
        let mut n = self.policy.native()?;
        if let Some(v) = self.convergence_tolerance {
            if !v.is_finite() || v <= 0.0 {
                return Err(invalid("Convergence tolerance must be finite and positive"));
            }
            n.convergence_tolerance = v;
        }
        n.settle_time_ms = duration_ms(self.settle_time)?;
        n.max_state_age_ms = self.max_state_age.native()?;
        n.wait_ceiling_ms = self.wait_ceiling.native()?;
        Ok(n)
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct CommandOptions {
    pub relative: bool,
    pub disable_limit_enforcement: bool,
    pub max_step: StepLimit,
}
impl CommandOptions {
    pub(crate) fn native(self) -> Result<ffi::dex_command_options_t> {
        let mut n = ffi::dex_command_options_t::default();
        unsafe { ffi::dex_command_options_init(&mut n) };
        n.relative = self.relative.into();
        n.disable_limit_enforcement = self.disable_limit_enforcement.into();
        n.max_step_rad = self.max_step.native()?;
        Ok(n)
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct TrajectoryOptions {
    /// None retains the native default. Some(0.0) explicitly disables tracking-error checking.
    pub max_tracking_error: Option<f64>,
    pub max_state_age: DurationLimit,
    pub max_step: StepLimit,
}
impl TrajectoryOptions {
    pub(crate) fn native(self) -> Result<ffi::dex_trajectory_options_t> {
        let mut n = ffi::dex_trajectory_options_t::default();
        unsafe { ffi::dex_trajectory_options_init(&mut n) };
        if let Some(v) = self.max_tracking_error {
            if !v.is_finite() || v < 0.0 {
                return Err(invalid("Tracking error must be finite and non-negative"));
            }
            n.max_tracking_error = v;
        }
        n.max_state_age_ms = self.max_state_age.native()?;
        n.max_step_rad = self.max_step.native()?;
        Ok(n)
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ChassisOptions {
    pub disable_sequential_steering: bool,
    pub steering_tolerance: Option<f64>,
    pub steering_wait: Option<Duration>,
    pub max_state_age: DurationLimit,
}
impl ChassisOptions {
    pub(crate) fn native(self) -> Result<ffi::dex_chassis_options_t> {
        let mut n = ffi::dex_chassis_options_t::default();
        unsafe { ffi::dex_chassis_options_init(&mut n) };
        n.disable_sequential_steering = self.disable_sequential_steering.into();
        if let Some(v) = self.steering_tolerance {
            if !v.is_finite() || v <= 0.0 {
                return Err(invalid("Steering tolerance must be finite and positive"));
            }
            n.steering_tolerance = v;
        }
        n.steering_wait_ms = self.steering_wait.map(millis).transpose()?.unwrap_or(0);
        n.max_state_age_ms = self.max_state_age.native()?;
        Ok(n)
    }
}

pub(crate) fn duration_ms(d: Duration) -> Result<u64> {
    let n = u64::try_from(d.as_millis()).map_err(|_| invalid("Duration is too large"))?;
    if n == u64::MAX || (!d.is_zero() && n == 0) {
        return Err(invalid(
            "Duration must be zero or at least 1 ms and below u64::MAX ms",
        ));
    }
    Ok(n)
}
