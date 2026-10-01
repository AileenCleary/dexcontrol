// Copyright (C) 2026 Dexmate Inc.
//
// This software is dual-licensed:
//
// 1. GNU Affero General Public License v3.0 (AGPL-3.0)
//    See LICENSE-AGPL for details
//
// 2. Commercial License
//    For commercial licensing terms, contact: contact@dexmate.ai

// Request a reboot of one robot control board.
//
// Require positional arm, torso, or chassis. The arm board controls both arms. A rebooting board stops controlling its joints, so ask for confirmation first and require typing yes; --yes skips the prompt and simulated runs do not prompt. Send the request and exit; do not wait for the board to come back online.
//
// Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.

#include "example.hpp"

int main(int argc, char **argv) {
    return examples::run(argc, argv, "Request a reboot of one robot control board. Require positional arm, torso, or chassis. The arm board controls both arms. A rebooting board stops controlling its joints, so ask for confirmation first and require typing yes; --yes skips the prompt and simulated runs do not prompt. Send the request and exit; do not wait for the board to come back online. Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.",
                         [](const examples::Args &args) {
        const auto board = args.positional_choice(0, {"arm", "torso", "chassis"}, "");
        examples::confirm_hazard(
            args, "rebooting the " + board + " control board" +
                      (board == "arm" ? " (it drives BOTH arms)" : "") +
                      ". The board stops controlling its joints while it restarts: make sure "
                      "nothing depends on them holding position.");
        auto robot = args.connect();
        robot.reboot(board);
        // Fire-and-forget: the board stops answering while it restarts, so a
        // clean return means the request was accepted, not that it is back.
        std::cout << board << ": reboot request sent\n";
        robot.close();
    }, {"yes"});
}
