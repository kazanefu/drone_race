use crate::playscene::drone;

use super::drone::*;
use bevy::prelude::*;

pub fn drone_cal(
    mut query: Query<(
        &mut DroneControl,
        &mut DroneStatus,
        &mut Transform,
        &mut Velocity,
        &mut Acceleration,
        &Gravity,
        &mut drone::Drag,
    )>,
    time: Res<Time>,
) {
    for (
        mut control,
        mut status,
        mut transform,
        mut velocity,
        mut acceleration,
        gravity,
        mut drag,
    ) in query.iter_mut()
    {
        let dt = time.delta_secs();
        // yaw
        transform.rotation *= Quat::from_rotation_y((control.yaw as f32) * dt);
        // pitch
        transform.rotation *= Quat::from_rotation_x((control.pitch as f32) * dt);
        // roll
        transform.rotation *= Quat::from_rotation_z((control.roll as f32) * dt);
        // 空気抵抗係数
        let drag_coefficient = 0.1; // this magic number is just for now
        // 空気抵抗
        drag.0 = velocity.0 * -drag_coefficient;
        // 加速度
        acceleration.0 = drag.0
            + gravity.0
            + (transform.up().as_vec3()
                * (control.throttle / status.mass) as f32
                * if control.boost { 2.0 } else { 1.0 });
        // ブーストをオフ
        status.is_boost = false;
        control.boost = false;
        // 速度
        velocity.0 += acceleration.0 * dt;
        // 位置
        transform.translation += velocity.0 * dt;
        // バッテリー消費
        status.battery -= status.battery_consumption_rate * control.throttle * dt as f64;
        // バッテリー切れ
        if status.battery <= 0.0 {
            status.is_crash = true;
        }
    }
}
