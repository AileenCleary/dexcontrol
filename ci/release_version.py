# Copyright (C) 2026 Dexmate Inc.
#
# This software is dual-licensed:
#
# 1. GNU Affero General Public License v3.0 (AGPL-3.0)
#    See LICENSE-AGPL for details
#
# 2. Commercial License
#    For commercial licensing terms, contact: contact@dexmate.ai
"""Canonical release tags and their Python and native package versions."""
import re
import sys
from typing import NamedTuple

NUMBER = r"(?:0|[1-9][0-9]*)"
BASE_PATTERN = rf"{NUMBER}\.{NUMBER}\.{NUMBER}"
RUST_VERSION_PATTERN = rf"{BASE_PATTERN}(?:-rc\.[1-9][0-9]*)?"
PYTHON_VERSION_PATTERN = rf"{BASE_PATTERN}(?:rc[1-9][0-9]*)?"


class ReleaseVersion(NamedTuple):
    rust: str
    python: str
    numeric: str
    prerelease: bool


def parse_tag(tag):
    match = re.fullmatch(rf"v({BASE_PATTERN})(?:-rc\.([1-9][0-9]*))?", tag)
    if match is None:
        raise ValueError("Expected vMAJOR.MINOR.PATCH or vMAJOR.MINOR.PATCH-rc.N (N >= 1)")
    base, candidate = match.groups()
    return ReleaseVersion(tag[1:], base + ("rc" + candidate if candidate else ""), base, candidate is not None)


if __name__ == "__main__":
    if len(sys.argv) != 2:
        sys.exit("Usage: release_version.py TAG")
    print(parse_tag(sys.argv[1]).python)
