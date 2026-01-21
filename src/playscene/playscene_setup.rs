use super::*;
use crate::game_rules::GameState;
use bevy::prelude::*;

pub struct PlaySceneSetupPlugin;

impl Plugin for PlaySceneSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scene)
            .add_systems(
                Update,
                (drone_cal::drone_cal).run_if(in_state(GameState::PlayScene)),
            )
            .add_systems(OnEnter(GameState::PlayScene), setup_scene)
            .add_systems(OnExit(GameState::PlayScene), system::cleanup_play_scene);
    }
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

    // Player Drone
    drone::spawn_drone(&mut commands);
}
