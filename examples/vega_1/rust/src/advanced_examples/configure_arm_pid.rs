// Copyright (C) 2026 Dexmate Inc.
//
// This software is dual-licensed:
//
// 1. GNU Affero General Public License v3.0 (AGPL-3.0)
//    See LICENSE-AGPL for details
//
// 2. Commercial License
//    For commercial licensing terms, contact: contact@dexmate.ai

//! Read or set arm PID multipliers.
//!
//! Require positional get or set. Address both arms by default; set sends seven multipliers, each defaulting to 1.0. Print service replies. PID writes can take about 40 s; the SDK waits up to 45 s for a reply. A timeout does not cancel a server-side write.
//!
//! Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.

use dexcontrol_examples::{component_name, invalid, Args, Result};

#[tokio::main]
async fn main() -> Result {
    let args = Args::parse(
        "Read or set arm PID multipliers. Require positional get or set. Address both arms by default; set sends seven multipliers, each defaulting to 1.0. Print service replies. PID writes can take about 40 s; the SDK waits up to 45 s for a reply. A timeout does not cancel a server-side write. Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.",
        &["p", "side"],
    );
    let action = args.positional(0, "");
    if action != "get" && action != "set" {
        return Err(invalid("action must be get or set"));
    }
    let p = args.numbers("p", vec![1.0; 7])?;
    let robot = args.connect(&[]).await?;
    dexcontrol_examples::run_robot(&robot, async {
        for side in args.sides()? {
            let arm = robot.joints(&component_name(side, "arm"))?;
            let result = if action == "get" {
                arm.get_pid()?
            } else {
                println!("Setting {side}_arm PID gains; this can take about 40 seconds...");
                arm.set_pid(&p)?
            };
            println!(
                "{side}_arm\n{}",
                serde_json::to_string_pretty(&serde_json::from_str::<serde_json::Value>(&result)?)?
            );
        }
        Ok(())
    })
    .await
}
