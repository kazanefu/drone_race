use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Drone;

#[derive(Component, Debug, Default)]
pub struct Velocity(pub Vec3);

#[derive(Component, Debug, Default)]
pub struct Acceleration(pub Vec3);

#[derive(Component, Debug)]
pub struct Gravity(pub Vec3);

#[derive(Component, Debug)]
pub struct CameraOffset(pub Vec3);

impl Default for CameraOffset {
    fn default() -> Self {
        Self(Vec3::new(0.0, 0.0, 0.5))
    }
}

#[derive(Component, Debug)]
pub struct CameraRotation(pub Quat);

impl Default for CameraRotation {
    fn default() -> Self {
        Self(Quat::from_rotation_x(-15_f32.to_radians()))
    }
}

#[derive(Component, Debug, Default)]
pub struct WithMainCamera;

impl Default for Gravity {
    fn default() -> Self {
        Self(Vec3::new(0.0, -9.81, 0.0))
    }
}

#[derive(Component, Debug, Default)]
pub struct Drag(pub Vec3);

#[derive(Component, Debug, Default)]
pub struct DroneStatus {
    pub mass: f64,
    pub battery: f64,
    pub throttle: f64,
    pub throttle_change_rate: f64,
    pub battery_consumption_rate: f64,
    pub is_boost: bool,
    pub is_crash: bool,
    pub roll_speed: f64,
    pub pitch_speed: f64,
    pub yaw_speed: f64,
}

#[derive(Component, Debug, Default)]
pub struct DroneControl {
    pub roll: f64,
    pub pitch: f64,
    pub yaw: f64,
    pub throttle: f64,
    pub boost: bool,
    pub crash: bool,
}

#[derive(Bundle, Default)]
pub struct DroneBundle {
    pub drone: Drone,
    pub transform: Transform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub gravity: Gravity,
    pub drag: Drag,
    pub status: DroneStatus,
    pub control: DroneControl,
    pub camera_offset: CameraOffset,
    pub camera_rotation: CameraRotation,
    pub with_main_camera: WithMainCamera,
}

pub fn spawn_drone(commands: &mut Commands) {
    commands.spawn(DroneBundle::default());
}
