use crate::game_rules::GameState;
use bevy::prelude::*;
use sysinfo::System;

/// Resource containing the real-time PC hardware status.
#[derive(Resource, Default, Debug)]
pub struct PcStatus {
    /// Percentage of memory used (0.0 to 1.0).
    pub memory_usage_rate: f64,
    /// Average CPU clock frequency in MHz.
    pub cpu_clock: f64,
    /// Estimated GPU clock frequency (currently a placeholder).
    pub gpu_clock: f64,
    /// Internal system handle for hardware metrics.
    pub system: System,
}

/// Marker component for the camera used during PC status calculation.
#[derive(Component)]
pub struct PCStatusCamera;

/// Plugin for managing the "Calculate PC" state and hardware monitoring.
pub struct PcStatusPlugin;

impl Plugin for PcStatusPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PcStatus::default())
            .add_plugins(super::cal_button::CalButtonPlugin)
            .add_plugins(super::next_scene::GoPlaySceneUIPlugin)
            .add_systems(OnEnter(GameState::CalculatePC), setup_pc_status_camera)
            .add_systems(
                Update,
                update_pc_status.run_if(in_state(crate::game_rules::GameState::CalculatePC)),
            );
    }
}

fn setup_pc_status_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        PCStatusCamera,
        DespawnOnExit(GameState::CalculatePC),
    ));
}

fn update_pc_status(
    mut pc_status: ResMut<PcStatus>,
    cal_pc_status_event: MessageReader<super::cal_button::CalPcStatusMessage>,
) {
    if cal_pc_status_event.is_empty() {
        return;
    }

    pc_status.system.refresh_memory();
    pc_status.system.refresh_cpu_all();

    // Memory usage rate
    let used_memory = pc_status.system.used_memory();
    let total_memory = pc_status.system.total_memory();
    pc_status.memory_usage_rate = used_memory as f64 / total_memory as f64;

    // Average CPU clock (MHz)
    let cpus = pc_status.system.cpus();
    if !cpus.is_empty() {
        let avg_clock: u64 =
            cpus.iter().map(|cpu| cpu.frequency()).sum::<u64>() / cpus.len() as u64;
        pc_status.cpu_clock = avg_clock as f64;
    }

    // GPU clock estimation or placeholder
    // Note: sysinfo doesn't easily provide GPU clock.
    // For now, setting a placeholder or leaving as 0.0 as per initial skeleton.
    pc_status.gpu_clock = 0.0;
}
