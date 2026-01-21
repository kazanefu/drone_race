use crate::playscene::drone::{DroneControl, DroneStatus};
use bevy::prelude::*;

pub fn handle_drone_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &mut DroneControl, &DroneStatus)>,
) {
    for (entity, mut control, status) in query.iter_mut() {
        // Reset or update based on input
        if keyboard_input.pressed(KeyCode::KeyW) {
            control.up_logic(&status);
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            control.down_logic(&status);
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
}
