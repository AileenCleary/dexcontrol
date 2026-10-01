// Copyright (C) 2026 Dexmate Inc.
//
// This software is dual-licensed:
//
// 1. GNU Affero General Public License v3.0 (AGPL-3.0)
//    See LICENSE-AGPL for details
//
// 2. Commercial License
//    For commercial licensing terms, contact: contact@dexmate.ai

// Read and print fingertip forces from one hand.
//
// Read the right hand once by default and report missing touch data. Optional repeated reads use a 0.1 s interval.
//
// Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.

#include "example.hpp"

int main(int argc, char **argv) {
    return examples::run(argc, argv, "Read and print fingertip forces from one hand. Read the right hand once by default and report missing touch data. Optional repeated reads use a 0.1 s interval. Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.",
                         [](const examples::Args &args) {
        const int samples = args.integer("samples", 1);
        const double period = args.number("period", 0.1);

        auto robot = args.connect();
        auto hand = robot.joints(args.side() + "_hand");
        for (int index = 0; index < samples; ++index) {
            const auto forces = hand.touch_forces();
            std::cout << (forces ? examples::format(*forces) : std::string("no touch sample")) << "\n";
            if (index + 1 < samples) examples::sleep_for(period);
        }
    }, {"period", "samples", "side"});
}
