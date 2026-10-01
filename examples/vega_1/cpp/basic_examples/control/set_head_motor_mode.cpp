// Copyright (C) 2026 Dexmate Inc.
//
// This software is dual-licensed:
//
// 1. GNU Affero General Public License v3.0 (AGPL-3.0)
//    See LICENSE-AGPL for details
//
// 2. Commercial License
//    For commercial licensing terms, contact: contact@dexmate.ai

// Enable or disable head motors.
//
// The optional positional mode is enable or disable; without it, send disable. Print the firmware reply; no head target is commanded.
//
// Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.

#include "example.hpp"

int main(int argc, char **argv) {
    return examples::run(argc, argv, "Enable or disable head motors. The optional positional mode is enable or disable; without it, send disable. Print the firmware reply; no head target is commanded. Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.",
                         [](const examples::Args &args) {
        const auto mode = args.positional_choice(0, {"enable", "disable"}, "disable");

        auto robot = args.connect();
        std::cout << robot.joints("head").set_mode(mode == "enable" ? dexcontrol::JointMode::Enable
                                                                     : dexcontrol::JointMode::Disable)
                  << "\n";
    }, {});
}
