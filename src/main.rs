mod game_rules;
mod input;
mod pc_status;
mod playscene;

use bevy::prelude::*;
use game_rules::{GameRulesPlugin, GameState};
use input::InputPlugin;
use pc_status::PcStatusPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PcStatusPlugin)
        .add_plugins(GameRulesPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(playscene::playscene_setup::PlaySceneSetupPlugin)
        .add_systems(Startup, (setup_camera, setup_scene))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}
