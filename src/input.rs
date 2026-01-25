use crate::playscene::drone::DroneControl;
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_drone_input);
    }
}

/// Map keyboard inputs to drone control commands.
pub fn handle_drone_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut DroneControl>,
) {
    for _control in query.iter_mut() {
        // Reset or update based on input
        if keyboard_input.pressed(KeyCode::KeyW) { /* Up logic */ }
        if keyboard_input.pressed(KeyCode::KeyS) { /* Down logic */ }
        // etc.
    }
}
