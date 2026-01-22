use bevy::prelude::*;
// まだbevy0.18に対応するrapier3dがリリースされてないので、いったんコメントアウト
// use bevy_rapier3d::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Drone;

// まだbevy0.18に対応するrapier3dがリリースされてないので、いったんコメントアウト
// 対応版が出たらrapier3dを使う
// #[derive(Bundle)]
// pub struct DronePhysicsBundle {
//     pub rigid_body: RigidBody,
//     pub velocity: Velocity,
//     pub collider: Collider,
//     pub damping: Damping,
// }

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
        // position offset
        Self(Vec3::new(0.0, 0.0, 0.5))
    }
}

#[derive(Component, Debug)]
pub struct CameraRotation(pub Quat);

impl Default for CameraRotation {
    fn default() -> Self {
        // rotation offset from x axis
        let offset = -15_f32.to_radians();
        Self(Quat::from_rotation_x(offset))
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
    pub battery_capacity: f64,
    pub battery: f64,
    pub throttle: f64,
    pub max_throttle: f64,
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

pub fn _spawn_drone_default(commands: &mut Commands) {
    commands.spawn(DroneBundle::default());
}
pub fn spawn_drone_with_pc_status(
    commands: &mut Commands,
    pc_status: Res<crate::pc_status::pc_status::PcStatus>,
) {
    commands.spawn(DroneBundle {
        status: DroneStatus {
            mass: 1.0 + pc_status.memory_usage_rate,
            battery_capacity: 100.0 * (1.0 - pc_status.memory_usage_rate),
            battery: 100.0 * (1.0 - pc_status.memory_usage_rate),
            max_throttle: 100.0 * (pc_status.cpu_clock + pc_status.gpu_clock) / 2.0,
            throttle: 0.0,
            throttle_change_rate: 100.0,
            battery_consumption_rate: (pc_status.cpu_clock + pc_status.gpu_clock) / 2.0,
            is_boost: false,
            is_crash: false,
            roll_speed: 1.0,
            pitch_speed: 1.0,
            yaw_speed: 1.0,
        },
        ..Default::default()
    });
}
