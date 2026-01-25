use bevy::prelude::*;

/// The overall state of the application.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    /// Initial screen.
    #[default]
    Home,
    /// Screen for hardware requirement calculation.
    CalculatePC,
    /// Active gameplay scene.
    PlayScene,
    /// Post-race result screen.
    Result,
}

pub struct GameRulesPlugin;

impl Plugin for GameRulesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
    }
}
