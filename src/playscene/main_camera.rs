use crate::game_rules::GameState;
use bevy::prelude::*;

/// Marker component for the main camera entity itself in the play scene.
#[derive(Component, Debug, Default)]
pub struct MainCamera;

#[derive(Bundle, Default)]
pub struct MainCameraBundle {
    pub camera: Camera3d,
    pub transform: Transform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub main_camera: MainCamera,
    pub despawn_on_exit: DespawnOnExit<GameState>,
}

pub fn setup_main_camera(commands: &mut Commands) {
    commands.spawn(MainCameraBundle {
        despawn_on_exit: DespawnOnExit(GameState::PlayScene),
        transform: Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

// update main camera position and rotation
pub fn update_main_camera(
    mut query: Query<&mut Transform, With<MainCamera>>,
    targets: Query<
        (
            &GlobalTransform,
            &super::drone::CameraOffset,
            &super::drone::CameraRotation,
        ),
        With<super::drone::WithMainCamera>,
    >,
) {
    // main camera is only one
    let (target, offset, rotation) = targets.single().expect("Multiple drones with main camera");
    for mut transform in query.iter_mut() {
        let rotated_offset = target.rotation() * offset.0;
        transform.translation = target.translation() + rotated_offset;
        transform.rotation = target.rotation() * rotation.0;
    }
}
