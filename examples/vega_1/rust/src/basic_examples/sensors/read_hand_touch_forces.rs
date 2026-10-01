// Copyright (C) 2026 Dexmate Inc.
//
// This software is dual-licensed:
//
// 1. GNU Affero General Public License v3.0 (AGPL-3.0)
//    See LICENSE-AGPL for details
//
// 2. Commercial License
//    For commercial licensing terms, contact: contact@dexmate.ai

//! Read and print fingertip forces from one hand.
//!
//! Read the right hand once by default and report missing touch data. Optional repeated reads use a 0.1 s interval.
//!
//! Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.

use dexcontrol_examples::{component_name, format, pause, Args, Result};

#[tokio::main]
async fn main() -> Result {
    let args = Args::parse(
        "Read and print fingertip forces from one hand. Read the right hand once by default and report missing touch data. Optional repeated reads use a 0.1 s interval. Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.",
        &["period", "samples", "side"],
    );
    let samples = args.integer("samples", 1)?;
    let period = args.duration("period", 0.1)?;
    let robot = args.connect(&[]).await?;
    dexcontrol_examples::run_robot(&robot, async {
        let hand = robot.joints(&component_name(args.side()?, "hand"))?;
        for _ in 0..samples {
            match hand.touch_forces() {
                Ok(Some(forces)) => println!("{}", format(&forces)),
                Ok(None) => println!("no touch sample"),
                Err(error) => return Err(error.into()),
            }
            pause(period).await;
        }
        Ok(())
    })
    .await
}
