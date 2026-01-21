use bevy::prelude::*;

// PlaySceneEntity is a marker component for entities that belong to the play scene.
#[derive(Component)]
pub struct PlaySceneEntity;

pub fn cleanup_play_scene(mut commands: Commands, query: Query<Entity, With<PlaySceneEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
