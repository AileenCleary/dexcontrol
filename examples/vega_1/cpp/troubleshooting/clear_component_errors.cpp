// Copyright (C) 2026 Dexmate Inc.
//
// This software is dual-licensed:
//
// 1. GNU Affero General Public License v3.0 (AGPL-3.0)
//    See LICENSE-AGPL for details
//
// 2. Commercial License
//    For commercial licensing terms, contact: contact@dexmate.ai

// Clear errors on all supported robot components.
//
// Send clear-error requests, print each component outcome, and exit nonzero if any outcome failed. This is a mutating maintenance operation.
//
// Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.

#include "example.hpp"
#include "json.hpp"

int main(int argc, char **argv) {
    return examples::run(argc, argv, "Clear errors on all supported robot components. Send clear-error requests, print each component outcome, and exit nonzero if any outcome failed. This is a mutating maintenance operation. Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.",
                         [](const examples::Args &args) {
        auto robot = args.connect();
        const auto report = examples::Json::parse(robot.clear_all_errors_json());
        const auto &outcomes = report["outcomes"].items();
        if (outcomes.empty()) {
            std::cout << "No clearable components are enabled for this robot.\n";
        }
        bool failed = false;
        for (const auto &outcome : outcomes) {
            const auto component = outcome["component"].as_string("?");
            if (outcome["success"].as_bool()) {
                std::cout << "✓ " << component << "\n";
            } else {
                failed = true;
                std::cout << "✗ " << component << ": " << outcome["error"].text("unknown error")
                          << "\n";
            }
        }
        if (failed) throw std::runtime_error("some components could not be cleared");
    }, {});
}
