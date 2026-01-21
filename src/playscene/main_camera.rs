use bevy::prelude::*;

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
}

pub fn setup_main_camera(commands: &mut Commands) {
    commands.spawn(MainCameraBundle::default());
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
