// Copyright (C) 2026 Dexmate Inc.
//
// This software is dual-licensed:
//
// 1. GNU Affero General Public License v3.0 (AGPL-3.0)
//    See LICENSE-AGPL for details
//
// 2. Commercial License
//    For commercial licensing terms, contact: contact@dexmate.ai

// Read and display one 2D LiDAR scan.
//
// Enable lidar_2d_front, wait up to 10 s for activity, and read one cached scan. Python can visualize XY points; native examples print range statistics. No scan file is saved.
//
// Prints numeric summaries/frame metadata; does not open a viewer.
//
// Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.

#include "example.hpp"

int main(int argc, char **argv) {
    return examples::run(argc, argv, "Read and display one 2D LiDAR scan. Enable lidar_2d_front, wait up to 10 s for activity, and read one cached scan. Python can visualize XY points; native examples print range statistics. No scan file is saved. Prints numeric summaries/frame metadata; does not open a viewer. Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.",
                         [](const examples::Args &args) {
        const auto sensor = args.text("sensor", "lidar_2d_front");

        auto robot = args.connect({sensor});
        if (!robot.wait_for_state(sensor, std::chrono::seconds{10})) {
            throw std::runtime_error(sensor + " did not become active");
        }
        const auto scan = robot.lidar_2d(sensor);
        size_t finite = 0;
        double low = INFINITY, high = -INFINITY;
        for (const double range : scan.ranges) {
            if (!std::isfinite(range)) continue;
            ++finite;
            low = std::min(low, range);
            high = std::max(high, range);
        }
        if (finite == 0) throw std::runtime_error("no finite returns in the scan");
        std::cout << std::fixed << std::setprecision(3) << "points=" << finite << " min=" << low
                  << " max=" << high << " (angle " << scan.info.angle_min << ".." << scan.info.angle_max
                  << " rad, range limit " << scan.info.range_max << " m)\n";
    }, {"sensor"});
}
