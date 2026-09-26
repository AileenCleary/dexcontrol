# Copyright (C) 2026 Dexmate Inc.
#
# This software is dual-licensed:
#
# 1. GNU Affero General Public License v3.0 (AGPL-3.0)
#    See LICENSE-AGPL for details
#
# 2. Commercial License
#    For commercial licensing terms, contact: contact@dexmate.ai
cmake_minimum_required(VERSION 3.16)
project(dexcontrol_customer_examples LANGUAGES CXX)
find_package(dexcontrol CONFIG REQUIRED)
add_executable(read_joint_positions read_joint_positions.cpp)
target_compile_features(read_joint_positions PRIVATE cxx_std_17)
target_link_libraries(read_joint_positions PRIVATE dexcontrol::dexcontrol)

# Keep the launcher path consistent for Makefiles, Ninja and multi-config generators.
set_target_properties(read_joint_positions PROPERTIES
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<0:>")
