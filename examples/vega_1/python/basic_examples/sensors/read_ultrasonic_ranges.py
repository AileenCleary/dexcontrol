# Copyright (C) 2026 Dexmate Inc.
#
# This software is dual-licensed:
#
# 1. GNU Affero General Public License v3.0 (AGPL-3.0)
#    See LICENSE-AGPL for details
#
# 2. Commercial License
#    For commercial licensing terms, contact: contact@dexmate.ai

"""Read and print ultrasonic ranges.

Enable ultrasonic, wait up to 5 s for activity, and print one observation by default. Optional repeated reads use a 0.1 s interval.

Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied."""

import time
from dataclasses import dataclass
from typing import Optional

import tyro
from dexcontrol import Robot
from typing_extensions import Annotated


@dataclass
class Args:
    """Command-line options."""

    sensor: str = "ultrasonic"
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
        enable_sensors=[args.sensor],
        simulation=args.simulated,
        profile=args.profile,
        config_file=args.config,
    ) as robot:
        sensor = robot.sensors.get_ultrasonic(args.sensor)
        if not sensor.wait_for_active(timeout=5.0):
            raise SystemExit(f"{args.sensor} did not become active")
        for index in range(args.samples):
            print(sensor.get_obs())
            if index + 1 < args.samples:
                time.sleep(args.period)


if __name__ == "__main__":
    main()
