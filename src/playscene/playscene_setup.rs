use super::*;
use crate::game_rules::GameState;
use bevy::prelude::*;

pub struct PlaySceneSetupPlugin;

// OnPlay is a state for the play scene.
// OnPlay::OffPlay is when On PlayScene, but drone is not spawned.
// OnPlay::OnPlay is when On PlayScene, and drone is spawned.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum OnPlay {
    #[default]
    OffPlay,
    OnPlay,
}

impl Plugin for PlaySceneSetupPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<OnPlay>()
            .add_plugins(system::TimerPlugin)
            .add_systems(Startup, setup_scene)
            .add_systems(
                Update,
                (
                    drone_cal::drone_cal,
                    main_camera::update_main_camera,
                    drone_control::handle_drone_input,
                )
                    .chain()
                    .run_if(in_state(OnPlay::OnPlay)),
            )
            .add_systems(
                OnEnter(GameState::PlayScene),
                (setup_scene, setup_drone).chain(),
            )
            .add_systems(OnEnter(OnPlay::OnPlay), setup_drone)
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
}

fn setup_drone(mut commands: Commands) {
    // Player Drone
    drone::spawn_drone(&mut commands);
    // Main Camera
    main_camera::setup_main_camera(&mut commands);
}
