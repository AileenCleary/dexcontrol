# Copyright (C) 2025-2026 Dexmate Inc.
#
# This software is dual-licensed:
#
# 1. GNU Affero General Public License v3.0 (AGPL-3.0)
#    See LICENSE-AGPL for details
#
# 2. Commercial License
#    For commercial licensing terms, contact: contact@dexmate.ai

"""Execute four planned relative end-effector moves with Enter prompts.

Using dexmotion at 250 Hz: move the right end effector +0.1 m in z in frame R_arm_j5; move the left +0.1 m in y; move the left +0.1 m in x in frame L_arm_j3; rotate the right +pi/4 around x. Prompt before each segment and stream both arm joint arrays. Uses a real Robot control connection; no simulation flag."""

import numpy as np
import tyro
from dexmotion.motion_manager import MotionManager

from dexcontrol import RateLimiter, Robot, ask


def move_real_robot(
    bot: Robot, qs_sample: list[list[float]], control_hz: float = 250
) -> None:
    """Move the real robot to a specific pose.

    Args:
        qs_sample: List of joint arrays to move
    """
    rate_limiter = RateLimiter(control_hz)
    for q in qs_sample:
        # Extract left and right arm joint arrays
        left_arm_joints = q[:7]
        right_arm_joints = q[7:14]

        # Set joint positions for both arms
        bot.left_arm.set_joint_pos(left_arm_joints)
        bot.right_arm.set_joint_pos(right_arm_joints)
        rate_limiter.sleep()


def main() -> None:
    """Execute four planned relative end-effector moves with Enter prompts.

    Using dexmotion at 250 Hz: move the right end effector +0.1 m in z in frame R_arm_j5; move the left +0.1 m in y; move the left +0.1 m in x in frame L_arm_j3; rotate the right +pi/4 around x. Prompt before each segment and stream both arm joint arrays. Uses a real Robot control connection; no simulation flag."""
    # Every other example uses the context manager; without it the session,
    # safety monitors and watchdog are never torn down on exit.
    with Robot() as bot:
        control_hz = 250
        components = ["left_arm", "right_arm", "head"]
        if bot.has_component("torso"):
            components.insert(2, "torso")
        initial_joint_pos = bot.get_joint_pos_dict(component=components)

        # Create task instance with initial joint configuration
        mm = MotionManager(initial_joint_configuration_dict=initial_joint_pos)

        moves = (
            (
                "move right end-effector by 0.1 meters in z direction",
                lambda: mm.right_arm.set_ee_pose(
                    pos=np.array([0.0, 0.0, 0.1]), relative=True, target_frame="R_arm_j5"
                ),
            ),
            (
                "move left end-effector by 0.1 meters in y direction",
                lambda: mm.left_arm.set_ee_pose(
                    pos=np.array([0.0, 0.1, 0.0]), relative=True
                ),
            ),
            (
                "move left end-effector by 0.1 meters in x direction",
                lambda: mm.left_arm.move_ee_xyz(
                    np.array([0.1, 0.0, 0.0]), target_frame="L_arm_j3"
                ),
            ),
            (
                "rotate right end-effector by 45 degrees around x-axis",
                lambda: mm.right_arm.move_ee_rpy(np.array([np.pi / 4, 0.0, 0.0])),
            ),
        )
        for description, plan in moves:
            qs_sample = plan()
            ask(f"Press Enter to {description}")
            move_real_robot(bot, qs_sample, control_hz)


if __name__ == "__main__":
    tyro.cli(main)
