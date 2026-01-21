use crate::playscene::drone;

use super::drone::*;
use bevy::prelude::*;

pub fn drone_cal(
    mut query: Query<(
        &DroneControl,
        &mut DroneStatus,
        &mut Transform,
        &mut Velocity,
        &mut Acceleration,
        &Gravity,
        &mut drone::Drag,
    )>,
    time: Res<Time>,
) {
    for (control, mut status, mut transform, mut velocity, mut acceleration, gravity, mut drag) in
        query.iter_mut()
    {
        // 空気抵抗
        drag.0 = velocity.0 * -0.1;
        // 加速度
        acceleration.0 = drag.0
            + gravity.0
            + (transform.up().as_vec3()
                * (control.throttle / status.mass) as f32
                * if control.boost { 2.0 } else { 1.0 });
        // 速度
        velocity.0 += acceleration.0 * time.delta_secs();
        // 位置
        transform.translation += velocity.0 * time.delta_secs();
        // バッテリー消費
        status.battery -=
            status.battery_consumption_rate * control.throttle * time.delta_secs() as f64;
        // バッテリー切れ
        if status.battery <= 0.0 {
            status.is_crash = true;
        }
    }
}
