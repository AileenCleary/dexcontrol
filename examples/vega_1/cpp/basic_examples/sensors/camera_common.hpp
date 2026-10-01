// Copyright (C) 2026 Dexmate Inc.
//
// This software is dual-licensed:
//
// 1. GNU Affero General Public License v3.0 (AGPL-3.0)
//    See LICENSE-AGPL for details
//
// 2. Commercial License
//    For commercial licensing terms, contact: contact@dexmate.ai

#pragma once

// Shared by the camera examples: subscribe to every requested stream, print
// the shape of each frame that arrives for a bounded number of samples, and
// unsubscribe on every exit path.

#include "example.hpp"

namespace examples {

inline const char *encoding_name(dexcontrol::FrameEncoding encoding) {
    switch (encoding) {
    case dexcontrol::FrameEncoding::Rgb8: return "rgb8";
    case dexcontrol::FrameEncoding::Bgr8: return "bgr8";
    case dexcontrol::FrameEncoding::Gray8: return "gray8";
    case dexcontrol::FrameEncoding::Depth32F: return "depth32f";
    }
    return "unknown";
}

inline void print_frames(dexcontrol::Robot &robot, const std::vector<std::string> &sensors,
                         const std::vector<std::string> &streams, int samples, double period) {
    std::vector<std::pair<std::string, std::string>> subscribed;
    for (const auto &sensor : sensors) {
        for (const auto &stream : streams.empty() ? robot.camera_streams(sensor) : streams) {
            subscribed.emplace_back(sensor, stream);
        }
    }
    for (const auto &[sensor, stream] : subscribed) robot.camera_subscribe(sensor, stream);
    try {
        for (int index = 0; index < samples; ++index) {
            for (const auto &[sensor, stream] : subscribed) {
                const auto frame = robot.camera_latest_frame(sensor, stream);
                if (!frame) continue;
                std::cout << sensor << "/" << stream << ": shape=(" << frame->height() << ", "
                          << frame->width() << ") " << encoding_name(frame->encoding()) << " "
                          << frame->size() << " bytes\n";
            }
            sleep_for(period);
        }
    } catch (...) {
        for (const auto &[sensor, stream] : subscribed) robot.camera_unsubscribe(sensor, stream);
        throw;
    }
    for (const auto &[sensor, stream] : subscribed) robot.camera_unsubscribe(sensor, stream);
}

}  // namespace examples
