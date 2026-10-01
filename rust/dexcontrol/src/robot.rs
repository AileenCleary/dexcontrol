// Copyright (C) 2026 Dexmate Inc.
//
// This software is dual-licensed:
//
// 1. GNU Affero General Public License v3.0 (AGPL-3.0)
//    See LICENSE-AGPL for details
//
// 2. Commercial License
//    For commercial licensing terms, contact: contact@dexmate.ai
//! Robot discovery, configuration, maintenance and model-selected chassis control.
use crate::*;
impl ConnectOptions<'_> {
    pub(crate) fn with_native<T>(
        &self,
        f: impl FnOnce(&ffi::dex_robot_options_t) -> Result<T>,
    ) -> Result<T> {
        if unsafe { ffi::dex_abi_version() } != ffi::EXPECTED_ABI_VERSION {
            return Err(invalid(
                "DexControl runtime ABI mismatch; install the matching SDK",
            ));
        }
        if self.profile.is_some() && self.config_file.is_some() {
            return Err(invalid("Choose profile or config_file, not both"));
        }
        let profile = self.profile.map(string).transpose()?;
        let config = self.config_file.map(string).transpose()?;
        let watchdog = self.watchdog_command.map(string).transpose()?;
        let sensors: Vec<CString> = self
            .enable_sensors
            .iter()
            .map(|s| string(s))
            .collect::<Result<_>>()?;
        let pointers: Vec<_> = sensors.iter().map(|s| s.as_ptr()).collect();
        let mut n = ffi::dex_robot_options_t::default();
        call(|_| unsafe { ffi::dex_robot_options_init(&mut n) })?;
        n.profile_utf8 = profile.as_ref().map_or(ptr::null(), |s| s.as_ptr());
        n.config_file_utf8 = config.as_ref().map_or(ptr::null(), |s| s.as_ptr());
        n.watchdog_command_utf8 = watchdog.as_ref().map_or(ptr::null(), |s| s.as_ptr());
        n.enable_sensors = pointers.as_ptr();
        n.enable_sensor_count = pointers.len();
        n.simulated = self.simulated.into();
        n.require_version_check = self.require_version_check.into();
        n.disable_watchdog = self.disable_watchdog.into();
        n.exit_on_termination = self.exit_on_termination.into();
        f(&n)
    }
    /// Resolve profile/configuration without connecting to hardware.
    pub fn resolved_config_json(&self) -> Result<String> {
        self.with_native(|n| text(|out, e| unsafe { ffi::dex_resolved_config_json(n, out, e) }))
    }
}
pub fn available_profiles_json() -> Result<String> {
    text(|out, e| unsafe { ffi::dex_available_profiles_json(out, e) })
}
pub fn profile_from_environment() -> Result<String> {
    text(|out, e| unsafe { ffi::dex_profile_from_environment(out, e) })
}
pub fn profile_for_robot_name(name: &str) -> Result<String> {
    let name = string(name)?;
    text(|out, e| unsafe { ffi::dex_profile_for_robot_name(name.as_ptr(), out, e) })
}
pub fn build_info_json() -> Result<String> {
    text(|out, e| unsafe { ffi::dex_build_info_json(out, e) })
}
impl Robot {
    /// Connect from normalized configuration JSON, for applications that compose
    /// their own robot profiles. Simulation never opens a hardware connection.
    pub fn from_resolved_json(json: &str, simulated: bool) -> Result<Self> {
        if unsafe { ffi::dex_abi_version() } != ffi::EXPECTED_ABI_VERSION {
            return Err(invalid(
                "DexControl runtime ABI mismatch; install the matching SDK",
            ));
        }
        let mut out = ptr::null_mut();
        let result = call(|e| unsafe {
            if simulated {
                ffi::dex_robot_create_simulated_from_resolved_json(
                    json.as_ptr(),
                    json.len(),
                    &mut out,
                    e,
                )
            } else {
                ffi::dex_robot_create_from_resolved_json(json.as_ptr(), json.len(), &mut out, e)
            }
        });
        let robot = NonNull::new(out).map(Self);
        result?;
        robot.ok_or_else(|| invalid("Runtime returned no robot"))
    }

    /// Retain a connection handle for concurrent use; closing either closes the connection.
    pub fn try_clone(&self) -> Result<Self> {
        call(|_| unsafe { ffi::dex_robot_retain(self.0.as_ptr()) })?;
        Ok(Self(self.0))
    }

    pub fn reconnect(&self) -> Result<()> {
        call(|e| unsafe { ffi::dex_robot_reconnect(self.0.as_ptr(), e) })
    }
    pub fn pause_heartbeat(&self) -> Result<()> {
        call(|e| unsafe { ffi::dex_robot_pause_heartbeat(self.0.as_ptr(), e) })
    }
    pub fn resume_heartbeat(&self) -> Result<()> {
        call(|e| unsafe { ffi::dex_robot_resume_heartbeat(self.0.as_ptr(), e) })
    }
    pub fn config_json(&self) -> Result<String> {
        text(|out, e| unsafe { ffi::dex_robot_config_json(self.0.as_ptr(), out, e) })
    }
    pub fn version_info_json(&self) -> Result<String> {
        text(|out, e| unsafe { ffi::dex_robot_version_info_json(self.0.as_ptr(), out, e) })
    }
    pub fn connection_report_json(&self) -> Result<String> {
        text(|out, e| unsafe { ffi::dex_robot_connection_report_json(self.0.as_ptr(), out, e) })
    }
    pub fn profile_name(&self) -> Result<String> {
        text(|out, e| unsafe { ffi::dex_robot_profile_name(self.0.as_ptr(), out, e) })
    }
    pub fn component_status_json(&self) -> Result<String> {
        text(|out, e| unsafe { ffi::dex_robot_component_status_json(self.0.as_ptr(), out, e) })
    }
    pub fn take_safety_events_json(&self) -> Result<String> {
        text(|out, e| unsafe { ffi::dex_robot_take_safety_events_json(self.0.as_ptr(), out, e) })
    }
    pub fn initialize_components_json(&self) -> Result<String> {
        text(|out, e| unsafe { ffi::dex_robot_initialize_components_json(self.0.as_ptr(), out, e) })
    }
    pub fn clear_all_errors_json(&self) -> Result<String> {
        text(|out, e| unsafe { ffi::dex_robot_clear_all_errors_json(self.0.as_ptr(), out, e) })
    }
    pub fn clear_error(&self, name: &str) -> Result<()> {
        let name = string(name)?;
        call(|e| unsafe { ffi::dex_robot_clear_error(self.0.as_ptr(), name.as_ptr(), e) })
    }
    pub fn reboot(&self, name: &str) -> Result<()> {
        let name = string(name)?;
        call(|e| unsafe { ffi::dex_robot_reboot(self.0.as_ptr(), name.as_ptr(), e) })
    }
    pub fn has_component(&self, name: &str) -> Result<bool> {
        let name = string(name)?;
        let mut yes = false;
        call(|e| unsafe {
            ffi::dex_robot_has_component(self.0.as_ptr(), name.as_ptr(), &mut yes, e)
        })?;
        Ok(yes)
    }
    pub fn has_sensor(&self, name: &str) -> Result<bool> {
        let name = string(name)?;
        let mut yes = false;
        call(|e| unsafe {
            ffi::dex_robot_has_sensor(self.0.as_ptr(), name.as_ptr(), &mut yes, e)
        })?;
        Ok(yes)
    }

    pub fn is_active(&self) -> Result<bool> {
        let mut yes = false;
        call(|_| unsafe { ffi::dex_robot_is_active(self.0.as_ptr(), &mut yes) })?;
        Ok(yes)
    }
    pub fn wait_for_active(&self, timeout: Duration) -> Result<bool> {
        let timeout = duration_ms(timeout)?;
        let mut yes = false;
        call(|e| unsafe { ffi::dex_robot_wait_for_active(self.0.as_ptr(), timeout, &mut yes, e) })?;
        Ok(yes)
    }
    pub fn wait_for_state(&self, name: &str, timeout: Duration) -> Result<bool> {
        let name = string(name)?;
        let timeout = duration_ms(timeout)?;
        let mut yes = false;
        call(|e| unsafe {
            ffi::dex_robot_wait_for_state(self.0.as_ptr(), name.as_ptr(), timeout, &mut yes, e)
        })?;
        Ok(yes)
    }
    pub fn wait_safety_event_json(&self, timeout: Duration) -> Result<String> {
        let timeout = duration_ms(timeout)?;
        text(|out, e| unsafe {
            ffi::dex_robot_wait_safety_event_json(self.0.as_ptr(), timeout, out, e)
        })
    }
    pub fn query_ntp_json(&self, samples: usize) -> Result<String> {
        text(|out, e| unsafe { ffi::dex_robot_query_ntp_json(self.0.as_ptr(), samples, out, e) })
    }
    pub fn query(&self, name: &str, payload: &[u8]) -> Result<Vec<u8>> {
        let name = string(name)?;
        buffer(|out, e| unsafe {
            ffi::dex_robot_query(
                self.0.as_ptr(),
                name.as_ptr(),
                payload.as_ptr(),
                payload.len(),
                out,
                e,
            )
        })
    }
    pub fn sensor_names(&self) -> Result<Vec<String>> {
        let mut n = 0;
        call(|_| unsafe { ffi::dex_robot_sensor_count(self.0.as_ptr(), &mut n) })?;
        (0..n)
            .map(|i| {
                text(|out, e| unsafe { ffi::dex_robot_sensor_name(self.0.as_ptr(), i, out, e) })
            })
            .collect()
    }
    /// None selects the unique enabled mobile base; the runtime rejects ambiguous selection.
    pub fn chassis(&self, name: Option<&str>) -> Result<Chassis<'_>> {
        Ok(Chassis {
            robot: self,
            name: name.map(string).transpose()?,
        })
    }
    pub fn torso_pitch(&self) -> Result<f64> {
        let mut v = 0.;
        call(|e| unsafe { ffi::dex_robot_torso_pitch(self.0.as_ptr(), &mut v, e) })?;
        Ok(v)
    }
    pub fn compensate_torso_pitch(&self, part: &str, values: &[f64]) -> Result<Vec<f64>> {
        let part = string(part)?;
        let mut out = vec![0.; values.len()];
        let mut n = 0;
        call(|e| unsafe {
            ffi::dex_robot_compensate_torso_pitch(
                self.0.as_ptr(),
                values.as_ptr(),
                values.len(),
                part.as_ptr(),
                out.as_mut_ptr(),
                out.len(),
                &mut n,
                e,
            )
        })?;
        if n > out.len() {
            return Err(invalid("Invalid compensated pose length"));
        }
        out.truncate(n);
        Ok(out)
    }
}
/// A named chassis view borrowing the robot connection.
pub struct Chassis<'a> {
    robot: &'a Robot,
    name: Option<CString>,
}
impl Chassis<'_> {
    fn name_ptr(&self) -> *const std::ffi::c_char {
        self.name.as_ref().map_or(ptr::null(), |s| s.as_ptr())
    }
    pub fn set_velocity(&self, vx: f64, vy: f64, wz: f64, options: ChassisOptions) -> Result<()> {
        let options = options.native()?;
        call(|e| unsafe {
            ffi::dex_robot_chassis_set_velocity_opt_named(
                self.robot.0.as_ptr(),
                self.name_ptr(),
                vx,
                vy,
                wz,
                &options,
                e,
            )
        })
    }
    pub fn stop(&self) -> Result<()> {
        call(|e| unsafe {
            ffi::dex_robot_chassis_stop_named(self.robot.0.as_ptr(), self.name_ptr(), e)
        })
    }
    pub fn drive_for(&self, vx: f64, vy: f64, wz: f64, duration: Duration, hz: f64) -> Result<()> {
        let duration = duration_ms(duration)?;
        call(|e| unsafe {
            ffi::dex_robot_chassis_drive_for_named(
                self.robot.0.as_ptr(),
                self.name_ptr(),
                vx,
                vy,
                wz,
                duration,
                hz,
                e,
            )
        })
    }
    pub fn set_motion_state(&self, steering: &[f64], wheels: &[f64]) -> Result<()> {
        call(|e| unsafe {
            ffi::dex_robot_chassis_set_motion_state_n_named(
                self.robot.0.as_ptr(),
                self.name_ptr(),
                steering.as_ptr(),
                steering.len(),
                wheels.as_ptr(),
                wheels.len(),
                e,
            )
        })
    }
    fn values(&self, kind: u32) -> Result<Vec<f64>> {
        array(|out, n, count, e| unsafe {
            ffi::dex_robot_chassis_values_named(
                self.robot.0.as_ptr(),
                self.name_ptr(),
                kind,
                out,
                n,
                count,
                e,
            )
        })
    }
    pub fn steering_angle(&self) -> Result<Vec<f64>> {
        self.values(0)
    }
    pub fn wheel_velocity(&self) -> Result<Vec<f64>> {
        self.values(1)
    }
    pub fn wheel_position(&self) -> Result<Vec<f64>> {
        self.values(2)
    }
}
