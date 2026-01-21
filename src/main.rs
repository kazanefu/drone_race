mod game_rules;
mod home_set;
mod home_ui;
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
        .add_systems(
            OnEnter(GameState::Home),
            (home_set::setup_camera, home_set::setup_scene),
        )
        .add_systems(OnExit(GameState::Home), home_set::despawn_camera)
        .run();
}
