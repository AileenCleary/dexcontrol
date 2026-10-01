// Copyright (C) 2026 Dexmate Inc.
//
// This software is dual-licensed:
//
// 1. GNU Affero General Public License v3.0 (AGPL-3.0)
//    See LICENSE-AGPL for details
//
// 2. Commercial License
//    For commercial licensing terms, contact: contact@dexmate.ai

//! Monitor frames from the configured head camera.
//!
//! Enable head_camera, subscribe to all advertised streams unless selected with --stream, and poll 200 times at 30 Hz before unsubscribing. The sensor name is configurable; no particular camera brand is required and no image files are saved.
//!
//! Prints numeric summaries/frame metadata; does not open a viewer.
//!
//! Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.

use dexcontrol_examples::{print_camera_frames, Args, Result};

#[tokio::main]
async fn main() -> Result {
    let args = Args::parse(
        "Monitor frames from the configured head camera. Enable head_camera, subscribe to all advertised streams unless selected with --stream, and poll 200 times at 30 Hz before unsubscribing. The sensor name is configurable; no particular camera brand is required and no image files are saved. Prints numeric summaries/frame metadata; does not open a viewer. Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.",
        &["period", "samples", "sensor", "stream"],
    );
    let sensor = args.text("sensor", "head_camera");
    let robot = args.connect(std::slice::from_ref(&sensor)).await?;
    dexcontrol_examples::run_robot(&robot, async {
        print_camera_frames(
            &robot,
            &[sensor],
            &args.list("stream"),
            args.integer("samples", 200)?,
            args.duration("period", 1.0 / 30.0)?,
        )
        .await?;
        Ok(())
    })
    .await
}
