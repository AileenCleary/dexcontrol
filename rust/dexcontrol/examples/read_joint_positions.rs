// Copyright (C) 2026 Dexmate Inc.
//
// This software is dual-licensed:
//
// 1. GNU Affero General Public License v3.0 (AGPL-3.0)
//    See LICENSE-AGPL for details
//
// 2. Commercial License
//    For commercial licensing terms, contact: contact@dexmate.ai
use dexcontrol::Robot;
fn main() -> dexcontrol::Result<()> {
    let robot = Robot::simulated("vega_1")?;
    for name in ["head", "left_arm", "right_arm", "torso"] {
        println!(
            "{name} joint positions (rad): {:.4?}",
            robot.joints(name)?.get_joint_pos()?
        );
    }
    robot.close()
}
