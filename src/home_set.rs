use crate::{GameState, home_ui::HomeUIPlugin};
use bevy::prelude::*;

pub struct HomeSetPlugin;

impl Plugin for HomeSetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HomeUIPlugin)
            .add_systems(OnEnter(GameState::Home), (setup_camera, setup_scene));
    }
}

// homeにおけるカメラのマーカー
#[derive(Component)]
pub struct HomeCamera;

// homeにおけるカメラスポーン
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        HomeCamera,
        //DespawnOnExit(GameState::Home),
    ));
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands.spawn((
        DespawnOnExit(GameState::Home),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.3, 0.3))),
    ));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
        DespawnOnExit(GameState::Home),
    ));
}
