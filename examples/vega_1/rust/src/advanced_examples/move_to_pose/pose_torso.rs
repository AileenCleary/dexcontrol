// Copyright (C) 2026 Dexmate Inc.
//
// This software is dual-licensed:
//
// 1. GNU Affero General Public License v3.0 (AGPL-3.0)
//    See LICENSE-AGPL for details
//
// 2. Commercial License
//    For commercial licensing terms, contact: contact@dexmate.ai

//! Move the torso to a named joint pose.
//!
//! Move to home and wait up to 10 s for tracked completion. No collision checking or automatic return is performed.
//!
//! Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.

use dexcontrol::Wait;
use dexcontrol_examples::{Args, Result};

#[tokio::main]
async fn main() -> Result {
    let args = Args::parse("Move the torso to a named joint pose. Move to home and wait up to 10 s for tracked completion. No collision checking or automatic return is performed. Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.", &["pose", "timeout"]);
    let robot = args.connect(&[]).await?;
    dexcontrol_examples::run_robot(&robot, async {
        let torso = robot.joints("torso")?;
        let target = robot
            .joints("torso")?
            .get_pose(&args.text("pose", "home"), false)?;
        let motion = torso.move_to_joint_pos_with(
            &target,
            dexcontrol::MotionOptions::default(),
            Wait::Timeout(args.duration("timeout", 10.0)?).into(),
        )?;
        println!("{:?}", motion.state()?);
        Ok(())
    })
    .await
}
