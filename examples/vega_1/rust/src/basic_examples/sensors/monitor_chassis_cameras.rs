// Copyright (C) 2026 Dexmate Inc.
//
// This software is dual-licensed:
//
// 1. GNU Affero General Public License v3.0 (AGPL-3.0)
//    See LICENSE-AGPL for details
//
// 2. Commercial License
//    For commercial licensing terms, contact: contact@dexmate.ai

//! Monitor frames from configured chassis cameras.
//!
//! Select base_*_camera sensors from the selected configuration unless --camera is supplied. Subscribe to all advertised streams unless --stream selects a subset; poll 200 times at 30 Hz, then unsubscribe. No image files are saved.
//!
//! Prints numeric summaries/frame metadata; does not open a viewer.
//!
//! Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.

use dexcontrol_examples::{print_camera_frames, Args, Result};

#[tokio::main]
async fn main() -> Result {
    let args = Args::parse(
        "Monitor frames from configured chassis cameras. Select base_*_camera sensors from the selected configuration unless --camera is supplied. Subscribe to all advertised streams unless --stream selects a subset; poll 200 times at 30 Hz, then unsubscribe. No image files are saved. Prints numeric summaries/frame metadata; does not open a viewer. Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.",
        &["camera", "period", "samples", "stream"],
    );
    let mut cameras = args.list("camera");
    if cameras.is_empty() {
        cameras = args.resolved_config()?["sensors"]
            .as_object()
            .into_iter()
            .flat_map(|s| s.keys())
            .filter(|name| name.starts_with("base_") && name.ends_with("_camera"))
            .cloned()
            .collect();
    }
    if cameras.is_empty() {
        return Err(dexcontrol_examples::invalid(
            "no chassis cameras are declared; pass --camera NAME",
        ));
    }
    let robot = args.connect(&cameras).await?;
    dexcontrol_examples::run_robot(&robot, async {
        print_camera_frames(
            &robot,
            &cameras,
            &args.list("stream"),
            args.integer("samples", 200)?,
            args.duration("period", 1.0 / 30.0)?,
        )
        .await?;
        Ok(())
    })
    .await
}
