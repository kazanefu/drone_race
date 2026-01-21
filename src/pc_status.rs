use bevy::prelude::*;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, System};

#[derive(Resource, Default)]
pub struct PcStatus {
    pub memory_usage_rate: f64,
    pub cpu_clock: f64,
    pub gpu_clock: f64,
    pub system: System,
}

pub struct PcStatusPlugin;

impl Plugin for PcStatusPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PcStatus::default()).add_systems(
            Update,
            update_pc_status.run_if(in_state(crate::game_rules::GameState::CalculatePC)),
        );
    }
}

fn update_pc_status(mut pc_status: ResMut<PcStatus>) {
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
