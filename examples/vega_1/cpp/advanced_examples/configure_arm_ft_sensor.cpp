// Copyright (C) 2026 Dexmate Inc.
//
// This software is dual-licensed:
//
// 1. GNU Affero General Public License v3.0 (AGPL-3.0)
//    See LICENSE-AGPL for details
//
// 2. Commercial License
//    For commercial licensing terms, contact: contact@dexmate.ai

// Read, enable, or disable arm force-torque sensor modes.
//
// Require positional get, enable, or disable. Address both arms by default and print service replies; this does not zero the force sensors.
//
// Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.

#include "example.hpp"

int main(int argc, char **argv) {
    return examples::run(argc, argv, "Read, enable, or disable arm force-torque sensor modes. Require positional get, enable, or disable. Address both arms by default and print service replies; this does not zero the force sensors. Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.",
                         [](const examples::Args &args) {
        const auto action = args.positional_choice(0, {"get", "enable", "disable"}, "");
        const auto side = args.choice("side", {"left", "right", "both"}, "both");
        const std::vector<std::string> sides =
            side == "both" ? std::vector<std::string>{"left", "right"} : std::vector<std::string>{side};

        auto robot = args.connect();
        for (const auto &name : sides) {
            auto arm = robot.joints(name + "_arm");
            const auto result = action == "get" ? arm.force_torque_sensor_mode()
                                                : arm.set_force_torque_sensor(action == "enable");
            std::cout << name << "_arm\n" << result << "\n";
        }
    }, {"side"});
}
