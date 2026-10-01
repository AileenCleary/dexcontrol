// Copyright (C) 2026 Dexmate Inc.
//
// This software is dual-licensed:
//
// 1. GNU Affero General Public License v3.0 (AGPL-3.0)
//    See LICENSE-AGPL for details
//
// 2. Commercial License
//    For commercial licensing terms, contact: contact@dexmate.ai

//! Read and print ultrasonic ranges.
//!
//! Enable ultrasonic, wait up to 5 s for activity, and print one observation by default. Optional repeated reads use a 0.1 s interval.
//!
//! Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.

use dexcontrol_examples::{pause, Args, Result};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result {
    let args = Args::parse(
        "Read and print ultrasonic ranges. Enable ultrasonic, wait up to 5 s for activity, and print one observation by default. Optional repeated reads use a 0.1 s interval. Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.",
        &["period", "samples", "sensor"],
    );
    let sensor = args.text("sensor", "ultrasonic");
    let samples = args.integer("samples", 1)?;
    let period = args.duration("period", 0.1)?;
    let robot = args.connect(std::slice::from_ref(&sensor)).await?;
    dexcontrol_examples::run_robot(&robot, async {
    if !robot
        .wait_for_state(&sensor, Duration::from_secs(5))?
    {
        return Err(dexcontrol_examples::invalid(format!(
            "{sensor} did not become active"
        )));
    }
    for _ in 0..samples {
        let state = robot.ultrasonic(&sensor, None)?.value;
        println!("front_left={:.3} front_right={:.3} back_left={:.3} back_right={:.3} m (timestamp {} ns)", state.front_left, state.front_right, state.back_left, state.back_right, state.timestamp_ns);
        pause(period).await;
    }
        Ok(())
    }).await
}
