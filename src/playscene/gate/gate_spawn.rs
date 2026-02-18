use bevy::prelude::*;

pub type GateID = i32;

#[derive(Component, Copy, Clone)]
pub enum Gate {
    Normal(GateID),
    SpeedUp,
    Goal,
}
impl Default for Gate {
    fn default() -> Self {
        Gate::Normal(0)
    }
}

#[derive(Component, Default, Copy, Clone)]
pub struct GateHitSize(f32);

#[derive(Bundle, Clone, Default)]
pub struct GateBundle<M: Material> {
    pub gate: Gate,
    transform: Transform,
    mesh: Mesh3d,
    material: MeshMaterial3d<M>,
    hit_size: GateHitSize,
}

pub fn spawn_gate(
    commands: &mut Commands,
    transform: Transform,
    id: GateID,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(GateBundle {
        gate: Gate::Normal(id),
        transform,
        mesh: Mesh3d(meshes.add(Sphere::default())),
        material: MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        hit_size: GateHitSize(1.0),
    });
}
