// Copyright (C) 2026 Dexmate Inc.
//
// This software is dual-licensed:
//
// 1. GNU Affero General Public License v3.0 (AGPL-3.0)
//    See LICENSE-AGPL for details
//
// 2. Commercial License
//    For commercial licensing terms, contact: contact@dexmate.ai

//! Monitor the IMU associated with a 3D LiDAR.
//!
//! Enable lidar_3d_front_imu, wait up to 10 s for activity, and print 100 observations at 0.05 s intervals. --sensor selects another declared IMU.
//!
//! Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.

use dexcontrol_examples::{format, pause, Args, Result};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result {
    let args = Args::parse(
        "Monitor the IMU associated with a 3D LiDAR. Enable lidar_3d_front_imu, wait up to 10 s for activity, and print 100 observations at 0.05 s intervals. --sensor selects another declared IMU. Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.",
        &["period", "samples", "sensor"],
    );
    let sensor = args.text("sensor", "lidar_3d_front_imu");
    let samples = args.integer("samples", 100)?;
    let period = args.duration("period", 0.05)?;
    let robot = args.connect(std::slice::from_ref(&sensor)).await?;
    dexcontrol_examples::run_robot(&robot, async {
        if !robot.wait_for_state(&sensor, Duration::from_secs(10))? {
            return Err(dexcontrol_examples::invalid(format!(
                "{sensor} did not become active"
            )));
        }
        for _ in 0..samples {
            let state = robot.imu(&sensor, None)?.value;
            println!(
                "acceleration={} angular_velocity={} orientation={}",
                format(&state.acceleration),
                format(&state.angular_velocity),
                format(&state.orientation)
            );
            pause(period).await;
        }
        Ok(())
    })
    .await
}
