use crate::playscene::drone::{DroneControl, DroneStatus};
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;

pub fn handle_drone_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_motion_events: MessageReader<MouseMotion>,
    mut mouse_wheel_events: MessageReader<MouseWheel>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut DroneControl, &mut DroneStatus)>,
) {
    for (mut control, mut status) in query.iter_mut() {
        // Reset or update based on input
        if keyboard_input.pressed(KeyCode::KeyW) {
            control.up_logic(&status);
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            control.down_logic(&status);
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            control.left_yaw_logic(&status);
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            control.right_yaw_logic(&status);
        }
        if mouse_buttons.just_pressed(MouseButton::Right) {
            control.boost_logic(&mut status);
        }

        for mouse_motion_event in mouse_motion_events.read() {
            control.roll_logic(&status, mouse_motion_event);
            control.pitch_logic(&status, mouse_motion_event);
        }
        for mouse_wheel_event in mouse_wheel_events.read() {
            status.throttle_change(&mouse_wheel_event);
        }

        // etc.
    }
}

impl DroneControl {
    fn up_logic(&mut self, status: &DroneStatus) {
        self.throttle = status.throttle;
    }
    fn down_logic(&mut self, status: &DroneStatus) {
        self.throttle = status.throttle * -1.0;
    }
    fn roll_logic(&mut self, status: &DroneStatus, mouse_motion_event: &MouseMotion) {
        self.roll = status.roll_speed * mouse_motion_event.delta.x as f64;
    }
    fn pitch_logic(&mut self, status: &DroneStatus, mouse_motion_event: &MouseMotion) {
        self.pitch = status.pitch_speed * mouse_motion_event.delta.y as f64;
    }
    fn left_yaw_logic(&mut self, status: &DroneStatus) {
        self.yaw = status.yaw_speed;
    }
    fn right_yaw_logic(&mut self, status: &DroneStatus) {
        self.yaw = status.yaw_speed * -1.0;
    }
    fn boost_logic(&mut self, status: &mut DroneStatus) {
        status.is_boost = true;
        self.boost = status.is_boost;
    }
}
impl DroneStatus {
    fn throttle_change(&mut self, mouse_wheel_event: &MouseWheel) {
        self.throttle += mouse_wheel_event.y as f64 * self.throttle_change_rate;
    }
}
