# Copyright (C) 2026 Dexmate Inc.
#
# This software is dual-licensed:
#
# 1. GNU Affero General Public License v3.0 (AGPL-3.0)
#    See LICENSE-AGPL for details
#
# 2. Commercial License
#    For commercial licensing terms, contact: contact@dexmate.ai

"""Read, activate, or deactivate the software E-stop.

Default positional action is status. activate requests software E-stop; deactivate clears it. Print the resulting observed status.

Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied."""

from dataclasses import dataclass
from typing import Literal, Optional

import tyro
from dexcontrol import Robot
from typing_extensions import Annotated


@dataclass
class Args:
    """Command-line options."""

    action: Annotated[
        Literal["status", "activate", "deactivate"], tyro.conf.Positional
    ] = "status"
    simulated: Annotated[bool, tyro.conf.arg(help="Use the in-process simulation")] = (
        False
    )
    profile: Annotated[Optional[str], tyro.conf.arg(help="Built-in robot profile")] = (
        None
    )
    config: Annotated[
        Optional[str], tyro.conf.arg(help="Custom robot configuration")
    ] = None


def print_status(status: dict) -> None:
    """Show observed E-stop sources without implying missing feedback is a sample."""
    print(f"E-stop:         {'ACTIVE' if status['engaged'] else 'Not active'}")
    print(f"Active sources: {', '.join(status['active_sources']) or 'None'}")
    print(f"Feedback:       {'Received' if status['state_observed'] else 'No event received (event-driven E-stop)'}")
    if status["engaged"]:
        print("Robot writes are blocked. Read operations remain available.")
        if any(source != "software" for source in status["active_sources"]):
            print("Release the active hardware E-stop before resuming control.")
        if status["software_estop_enabled"]:
            print("Clear the software E-stop explicitly before resuming control.")


def main() -> None:
    args = tyro.cli(Args, description=__doc__, config=(tyro.conf.FlagCreatePairsOff,))
    with Robot(
        simulation=args.simulated, profile=args.profile, config_file=args.config
    ) as robot:
        if args.action == "activate":
            robot.estop.activate()
        elif args.action == "deactivate":
            robot.estop.deactivate()
        print_status(robot.estop.get_status())


if __name__ == "__main__":
    main()
