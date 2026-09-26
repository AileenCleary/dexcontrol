# Copyright (C) 2026 Dexmate Inc.
#
# This software is dual-licensed:
#
# 1. GNU Affero General Public License v3.0 (AGPL-3.0)
#    See LICENSE-AGPL for details
#
# 2. Commercial License
#    For commercial licensing terms, contact: contact@dexmate.ai
"""Read the same four components as the C++ and Rust examples, in simulation."""
from dexcontrol import Robot

with Robot(profile="vega_1", simulation=True) as robot:
    for name in ("head", "left_arm", "right_arm", "torso"):
        values = robot.joints(name).get_joint_pos()
        print(f"{name} joint positions (rad): " + ", ".join(f"{v:.4f}" for v in values))
