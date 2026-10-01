// Copyright (C) 2026 Dexmate Inc.
//
// This software is dual-licensed:
//
// 1. GNU Affero General Public License v3.0 (AGPL-3.0)
//    See LICENSE-AGPL for details
//
// 2. Commercial License
//    For commercial licensing terms, contact: contact@dexmate.ai

// Read and print ultrasonic ranges.
//
// Enable ultrasonic, wait up to 5 s for activity, and print one observation by default. Optional repeated reads use a 0.1 s interval.
//
// Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.

#include "example.hpp"

int main(int argc, char **argv) {
    return examples::run(argc, argv, "Read and print ultrasonic ranges. Enable ultrasonic, wait up to 5 s for activity, and print one observation by default. Optional repeated reads use a 0.1 s interval. Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.",
                         [](const examples::Args &args) {
        const auto sensor = args.text("sensor", "ultrasonic");
        const int samples = args.integer("samples", 1);
        const double period = args.number("period", 0.1);

        auto robot = args.connect({sensor});
        if (!robot.wait_for_state(sensor, std::chrono::seconds{5})) {
            throw std::runtime_error(sensor + " did not become active");
        }
        for (int index = 0; index < samples; ++index) {
            const auto reading = robot.ultrasonic(sensor);
            std::cout << std::fixed << std::setprecision(3) << "front_left=" << reading.front_left
                      << " front_right=" << reading.front_right << " back_left=" << reading.back_left
                      << " back_right=" << reading.back_right << " m (timestamp " << reading.timestamp_ns
                      << " ns)\n";
            if (index + 1 < samples) examples::sleep_for(period);
        }
    }, {"period", "samples", "sensor"});
}
