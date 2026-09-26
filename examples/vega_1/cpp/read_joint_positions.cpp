// Copyright (C) 2026 Dexmate Inc.
//
// This software is dual-licensed:
//
// 1. GNU Affero General Public License v3.0 (AGPL-3.0)
//    See LICENSE-AGPL for details
//
// 2. Commercial License
//    For commercial licensing terms, contact: contact@dexmate.ai
#include <dexcontrol/dexcontrol.hpp>
#include <iomanip>
#include <iostream>
int main() {
    try {
        auto robot = dexcontrol::Robot::simulated("vega_1");
        for (const auto *name : {"head", "left_arm", "right_arm", "torso"}) {
            std::cout << name << " joint positions (rad):";
            for (double value : robot.joints(name).get_joint_pos())
                std::cout << ' ' << std::fixed << std::setprecision(4) << value;
            std::cout << '\n';
        }
        robot.close();
    } catch (const std::exception &error) {
        std::cerr << error.what() << '\n'; return 1;
    }
}
