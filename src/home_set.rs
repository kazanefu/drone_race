use crate::GameState;
use bevy::prelude::*;

pub struct HomeSetPlugin;

impl Plugin for HomeSetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Home), (setup_camera, setup_scene))
            .add_systems(OnExit(GameState::Home), despawn_all_home_entities);
    }
}

// homeにあるもののマーカー
#[derive(Component)]
pub struct HomeMarker;

#[derive(Component)]
pub struct HomeCamera;

// homeにおけるカメラスポーン
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        HomeMarker,
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
        HomeMarker,
    ));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
        HomeMarker,
    ));
}

fn despawn_all_home_entities(mut commands: Commands, home_marker: Query<Entity, With<HomeMarker>>) {
    for entity in home_marker.iter() {
        commands.entity(entity).despawn();
    }
}
