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
