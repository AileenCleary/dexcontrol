// Copyright (C) 2026 Dexmate Inc.
//
// This software is dual-licensed:
//
// 1. GNU Affero General Public License v3.0 (AGPL-3.0)
//    See LICENSE-AGPL for details
//
// 2. Commercial License
//    For commercial licensing terms, contact: contact@dexmate.ai

//! Estimate robot clock offset and network round-trip time.
//!
//! Use a read-only DiagnosticClient to collect 30 time-query samples and report server-minus-client offset, round-trip time, and replies. Does not synchronize or change either clock; no simulation connection is supported.
//!
//! Pretty-printed diagnostic JSON.

use dexcontrol_examples::{Args, Result};

#[tokio::main]
async fn main() -> Result {
    let args = Args::parse(
        "Estimate robot clock offset and network round-trip time. Use a read-only DiagnosticClient to collect 30 time-query samples and report server-minus-client offset, round-trip time, and replies. Does not synchronize or change either clock; no simulation connection is supported. Pretty-printed diagnostic JSON.",
        &["samples"],
    );
    let client = args.diagnostics().await?;
    let stats = serde_json::from_str::<serde_json::Value>(
        &client.query_ntp_json(args.integer("samples", 30)?)?,
    )?;
    println!("{}", serde_json::to_string_pretty(&stats)?);
    client.close()?;
    Ok(())
}
