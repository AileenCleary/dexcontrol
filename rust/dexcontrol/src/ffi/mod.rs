// Copyright (C) 2026 Dexmate Inc.
//
// This software is dual-licensed:
//
// 1. GNU Affero General Public License v3.0 (AGPL-3.0)
//    See LICENSE-AGPL for details
//
// 2. Commercial License
//    For commercial licensing terms, contact: contact@dexmate.ai

//! Private C ABI declarations for the safe public API.
#![allow(dead_code)]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(rustdoc::broken_intra_doc_links, rustdoc::bare_urls)]
include!("bindings.rs");
include!("constants.rs");
pub const EXPECTED_ABI_VERSION: u32 = DEX_ABI_VERSION;
