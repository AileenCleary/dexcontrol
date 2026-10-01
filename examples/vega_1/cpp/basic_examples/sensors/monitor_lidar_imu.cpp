// Copyright (C) 2026 Dexmate Inc.
//
// This software is dual-licensed:
//
// 1. GNU Affero General Public License v3.0 (AGPL-3.0)
//    See LICENSE-AGPL for details
//
// 2. Commercial License
//    For commercial licensing terms, contact: contact@dexmate.ai

// Monitor the IMU associated with a 3D LiDAR.
//
// Enable lidar_3d_front_imu, wait up to 10 s for activity, and print 100 observations at 0.05 s intervals. --sensor selects another declared IMU.
//
// Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.

#include "example.hpp"

int main(int argc, char **argv) {
    return examples::run(argc, argv, "Monitor the IMU associated with a 3D LiDAR. Enable lidar_3d_front_imu, wait up to 10 s for activity, and print 100 observations at 0.05 s intervals. --sensor selects another declared IMU. Uses a Robot control connection: compatible arm modes may be enabled during startup, and shutdown requests a stop. Real hardware is selected unless --simulated is supplied.",
                         [](const examples::Args &args) {
        const auto sensor = args.text("sensor", "lidar_3d_front_imu");
        const int samples = args.integer("samples", 100);
        const double period = args.number("period", 0.05);

        auto robot = args.connect({sensor});
        if (!robot.wait_for_state(sensor, std::chrono::seconds{10})) {
            throw std::runtime_error(sensor + " did not become active");
        }
        for (int index = 0; index < samples; ++index) {
            const auto imu = robot.imu(sensor);
            std::cout << "acceleration=" << examples::format(imu.acceleration)
                      << " angular_velocity=" << examples::format(imu.angular_velocity)
                      << " orientation=" << examples::format(imu.orientation, 4) << "\n";
            if (index + 1 < samples) examples::sleep_for(period);
        }
    }, {"period", "samples", "sensor"});
}
