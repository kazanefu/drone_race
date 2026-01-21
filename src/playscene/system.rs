use super::*;
use bevy::prelude::*;

// PlaySceneEntity is a marker component for entities that belong to the play scene.
#[derive(Component)]
pub struct PlaySceneEntity;

pub fn cleanup_play_scene(mut commands: Commands, query: Query<Entity, With<PlaySceneEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(Resource, Default)]
pub struct RaceTimer(pub Timer);

pub fn update_race_timer(mut timer: ResMut<RaceTimer>, time: Res<Time>) {
    timer.0.tick(time.delta());
}

pub fn reset_race_timer(mut timer: ResMut<RaceTimer>) {
    timer.0.reset();
}

impl RaceTimer {
    pub fn get_race_time(&self) -> f64 {
        self.0.elapsed_secs() as f64
    }
}

pub struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RaceTimer>()
            .add_systems(OnEnter(playscene_setup::OnPlay::OnPlay), reset_race_timer)
            .add_systems(
                Update,
                update_race_timer.run_if(in_state(playscene_setup::OnPlay::OnPlay)),
            );
    }
}
