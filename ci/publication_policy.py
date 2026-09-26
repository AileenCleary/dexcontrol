#!/usr/bin/env python3
# Copyright (C) 2026 Dexmate Inc.
#
# This software is dual-licensed:
#
# 1. GNU Affero General Public License v3.0 (AGPL-3.0)
#    See LICENSE-AGPL for details
#
# 2. Commercial License
#    For commercial licensing terms, contact: contact@dexmate.ai
"""Public content rules; private CI additionally scans internal documentation fingerprints."""
import re

RULES = {
    "credential": rb"gh[pousr]_[A-Za-z0-9]{30,}|github_pat_[A-Za-z0-9_]{50,}|AKIA[0-9A-Z]{16}",
    "private key": rb"-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----[\r\n]+[A-Za-z0-9+/=\r\n]{40,}",
    "private registry": rb"git\.dexmate\.us",
}
BINARY_RULES = {
    "FFmpeg in customer runtime": rb"libavcodec/|avcodec_open2|FFmpeg version",
    "unused robot geometry": rb"<mesh\s+filename=|<inertial(?:\s|>)|<inertia\s+ixx=",
    "test instrumentation": rb"_sim_[A-Za-z_]+|_drop_stats|_log_selftest",
    "private implementation name": rb"(?:dexcomm|dexcontrol_core|dexcontrol_transport|dexcontrol_protocol|dexcontrol_dexcomm|dexbot_utils|dexbot_model)::",
    "private source path": rb"(?:crates/dexcontrol[^/\s]*/|dexcomm[^/\s]*/src/|dexbot[-_](?:model|utils)[^/\s]*/src/)",
    "build path": rb"/(?:home|Users)/[^/\s\x00]+/(?:projects|work|\.cargo|\.rustup)/",
}


def findings(data, *, binary=False):
    # Match both regular strings and UTF-16 strings embedded in native data.
    forms = (data, data.replace(b"\x00", b""))
    rules = {**RULES, **(BINARY_RULES if binary else {})}
    return [name for name, pattern in rules.items() if any(re.search(pattern, value) for value in forms)]
