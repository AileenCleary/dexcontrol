# Copyright (C) 2026 Dexmate Inc.
#
# This software is dual-licensed:
#
# 1. GNU Affero General Public License v3.0 (AGPL-3.0)
#    See LICENSE-AGPL for details
#
# 2. Commercial License
#    For commercial licensing terms, contact: contact@dexmate.ai

"""Request a reboot of one robot control board.

Require positional arm, torso, or chassis. The arm board controls both arms. Send the request and exit; do not wait for the board to come back online.

Rebooting a board drops its joints out of control while it restarts, so the script asks for a typed "yes" first unless --yes is supplied. Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied; simulated runs skip the confirmation prompt."""

import sys
from dataclasses import dataclass
from typing import Literal, Optional

import tyro
from dexcontrol import Robot, confirm
from typing_extensions import Annotated


@dataclass
class Args:
    """Command-line options."""

    board: Annotated[Literal["arm", "torso", "chassis"], tyro.conf.Positional]
    yes: Annotated[
        bool, tyro.conf.arg(help="Skip the interactive reboot confirmation")
    ] = False
    simulated: Annotated[bool, tyro.conf.arg(help="Use the in-process simulation")] = (
        False
    )
    profile: Annotated[Optional[str], tyro.conf.arg(help="Built-in robot profile")] = (
        None
    )
    config: Annotated[
        Optional[str], tyro.conf.arg(help="Custom robot configuration")
    ] = None


def confirm_reboot(args: Args) -> None:
    print(
        f"WARNING: rebooting the {args.board} control board. Its joints are not "
        "controlled while it restarts; make sure the robot is in a safe, "
        "supported posture.",
        file=sys.stderr,
    )
    if args.yes or args.simulated:
        return
    if not confirm(f"Reboot the {args.board} board?", expected="yes"):
        print("Aborted: no reboot was requested.", file=sys.stderr)
        raise SystemExit(1)


def main() -> None:
    args = tyro.cli(Args, description=__doc__, config=(tyro.conf.FlagCreatePairsOff,))
    confirm_reboot(args)
    with Robot(
        simulation=args.simulated, profile=args.profile, config_file=args.config
    ) as robot:
        robot.reboot_component(args.board)
        # Fire-and-forget: the board stops answering while it restarts, so a
        # clean return means the request was accepted, not that it is back.
        print(f"{args.board}: reboot request sent")


if __name__ == "__main__":
    main()
