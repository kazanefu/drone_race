use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Drone;

#[derive(Component, Debug, Default)]
pub struct Velocity(pub Vec3);

#[derive(Component, Debug, Default)]
pub struct Acceleration(pub Vec3);

#[derive(Component, Debug)]
pub struct Gravity(pub Vec3);

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
    pub battery_consumption_rate: f64,
    pub is_boost: bool,
    pub is_crash: bool,
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
}

pub fn spawn_drone(commands: &mut Commands) {
    commands.spawn(DroneBundle::default());
}
