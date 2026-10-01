// Copyright (C) 2026 Dexmate Inc.
//
// This software is dual-licensed:
//
// 1. GNU Affero General Public License v3.0 (AGPL-3.0)
//    See LICENSE-AGPL for details
//
// 2. Commercial License
//    For commercial licensing terms, contact: contact@dexmate.ai

//! Read, activate, or deactivate the software E-stop.
//!
//! Default positional action is status. activate requests software E-stop; deactivate clears it. Print the resulting observed status.
//!
//! Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.

use dexcontrol_examples::{invalid, Args, Result};

#[tokio::main]
async fn main() -> Result {
    let args = Args::parse(
        "Read, activate, or deactivate the software E-stop. Default positional action is status. activate requests software E-stop; deactivate clears it. Print the resulting observed status. Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.",
        &[],
    );
    let action = args.positional(0, "status");
    if !["status", "activate", "deactivate"].contains(&action.as_str()) {
        return Err(invalid("action must be status, activate, or deactivate"));
    }
    let robot = args.connect(&[]).await?;
    dexcontrol_examples::run_robot(&robot, async {
        if action == "activate" {
            robot.set_software_estop(true)?;
        }
        if action == "deactivate" {
            robot.set_software_estop(false)?;
        }
        let engagement = robot.estop_status()?;
        let mut sources = Vec::new();
        {
            let state = &engagement;
            for (active, label) in [
                (state.software_estop_enabled, "software"),
                (state.left_base_estop_enabled, "left base hardware"),
                (state.right_base_estop_enabled, "right base hardware"),
                (state.torso_estop_enabled, "torso hardware"),
                (state.remote_estop_enabled, "remote hardware"),
            ] {
                if active {
                    sources.push(label);
                }
            }
        }
        println!(
            "E-stop:         {}",
            if engagement.engaged {
                "ACTIVE"
            } else {
                "Not active"
            }
        );
        println!(
            "Active sources: {}",
            if sources.is_empty() {
                "None".into()
            } else {
                sources.join(", ")
            }
        );
        println!(
            "Feedback:       {}",
            if engagement.state_observed {
                "Received"
            } else {
                "No event received (event-driven E-stop)"
            }
        );
        if engagement.engaged {
            println!("Robot writes are blocked. Read operations remain available.");
        }
        Ok(())
    })
    .await
}
