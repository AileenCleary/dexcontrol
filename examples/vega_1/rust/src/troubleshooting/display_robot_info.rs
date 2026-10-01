// Copyright (C) 2026 Dexmate Inc.
//
// This software is dual-licensed:
//
// 1. GNU Affero General Public License v3.0 (AGPL-3.0)
//    See LICENSE-AGPL for details
//
// 2. Commercial License
//    For commercial licensing terms, contact: contact@dexmate.ai

//! Display robot topology, observed state, and server diagnostics.
//!
//! Use a read-only DiagnosticClient and a 0.25 s state observation window, then print the snapshot. Optional --enable-sensor includes declared optional sensors. Does not enable motors or issue stop/motion commands; no simulation connection is supported.
//!
//! Formatted native diagnostic tables.

use dexcontrol_examples::{Args, Result};

#[tokio::main]
async fn main() -> Result {
    let args = Args::parse(
        "Display robot topology, observed state, and server diagnostics. Use a read-only DiagnosticClient and a 0.25 s state observation window, then print the snapshot. Optional --enable-sensor includes declared optional sensors. Does not enable motors or issue stop/motion commands; no simulation connection is supported. Formatted native diagnostic tables.",
        &["settle-timeout"],
    );
    let client = args.diagnostics().await?;
    let snapshot = client.snapshot_json(args.duration("settle-timeout", 0.25)?)?;
    print!(
        "{}",
        dexcontrol_examples::diagnostic_display::render(
            &serde_json::from_str::<serde_json::Value>(&snapshot)?
        )
    );
    client.close()?;
    Ok(())
}
