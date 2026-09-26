// Copyright (C) 2026 Dexmate Inc.
//
// This software is dual-licensed:
//
// 1. GNU Affero General Public License v3.0 (AGPL-3.0)
//    See LICENSE-AGPL for details
//
// 2. Commercial License
//    For commercial licensing terms, contact: contact@dexmate.ai
use dexcontrol::{MotionOptions, MotionState, Robot, Wait};
use std::time::Duration;

#[test]
fn component_owns_connection_and_motion_completes() {
    let robot = Robot::simulated("vega_1").unwrap();
    let head = robot.joints("head").unwrap();
    drop(robot);
    assert_eq!(head.name().unwrap(), "head");
    let target = head.get_joint_pos().unwrap();
    assert_eq!(target.len(), 3);
    let motion = head
        .move_to_joint_pos(&target, MotionOptions::default())
        .unwrap();
    drop(head);
    motion.wait(Duration::from_secs(5)).unwrap();
    assert_eq!(motion.state().unwrap(), MotionState::Succeeded);
}

#[test]
fn errors_are_preserved_without_sending_invalid_commands() {
    let robot = Robot::simulated("vega_1").unwrap();
    assert!(robot.joints("head\0oops").is_err());
    assert!(robot.joints("missing").is_err());
    let head = robot.joints("head").unwrap();
    let target = head.get_joint_pos().unwrap();
    assert!(head
        .move_to_joint_pos(
            &target,
            MotionOptions {
                velocity_scale: Some(f64::NAN),
                ..Default::default()
            }
        )
        .is_err());
    assert!(head
        .set_joint_pos(&target, Wait::Timeout(Duration::ZERO))
        .is_err());
    robot.set_software_estop(true).unwrap();
    let error = head.set_joint_pos(&target, Wait::NoWait).unwrap_err();
    assert_eq!(error.code, 8);
    assert!(error.message.contains("E-stop"));
    assert!(head.get_joint_pos().is_ok());
    robot.close().unwrap();
    assert!(head.get_joint_pos().is_err());
}

#[test]
fn shared_handles_support_concurrent_reads() {
    let robot = Robot::simulated("vega_1").unwrap();
    let head = std::sync::Arc::new(robot.joints("head").unwrap());
    let readers: Vec<_> = (0..4)
        .map(|_| {
            let head = head.clone();
            std::thread::spawn(move || head.get_joint_pos().unwrap())
        })
        .collect();
    for reader in readers {
        assert_eq!(reader.join().unwrap().len(), 3);
    }
    robot.close().unwrap();
}
