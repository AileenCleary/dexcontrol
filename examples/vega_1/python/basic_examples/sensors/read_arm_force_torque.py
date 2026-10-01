# Copyright (C) 2026 Dexmate Inc.
#
# This software is dual-licensed:
#
# 1. GNU Affero General Public License v3.0 (AGPL-3.0)
#    See LICENSE-AGPL for details
#
# 2. Commercial License
#    For commercial licensing terms, contact: contact@dexmate.ai

"""Read and print one arm force-torque observation.

Wait up to 5 s for the right-arm wrench stream and print one observation by default. Optional repeated reads use a 0.1 s interval; no calibration or zeroing is requested.

Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied."""

import time
from dataclasses import dataclass
from typing import Literal, Optional

import tyro
from dexcontrol import Robot
from typing_extensions import Annotated


@dataclass
class Args:
    """Command-line options."""

    side: Literal["left", "right"] = "right"
    samples: int = 1
    period: float = 0.1
    simulated: Annotated[bool, tyro.conf.arg(help="Use the in-process simulation")] = (
        False
    )
    profile: Annotated[Optional[str], tyro.conf.arg(help="Built-in robot profile")] = (
        None
    )
    config: Annotated[
        Optional[str], tyro.conf.arg(help="Custom robot configuration")
    ] = None


def main() -> None:
    args = tyro.cli(Args, description=__doc__, config=(tyro.conf.FlagCreatePairsOff,))
    with Robot(
        simulation=args.simulated, profile=args.profile, config_file=args.config
    ) as robot:
        wrench = robot.sensors.get_wrench(f"{args.side}_arm")
        if not wrench.wait_for_active(timeout=5.0):
            raise SystemExit("wrench state did not become active")
        for index in range(args.samples):
            print(wrench.get_wrench_state())
            if index + 1 < args.samples:
                time.sleep(args.period)


if __name__ == "__main__":
    main()
