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
        .add_plugins(PcStatusPlugin) // pc status management
        .add_plugins(GameRulesPlugin) // game state management
        .add_plugins(InputPlugin)
        .add_plugins(playscene::playscene_setup::PlaySceneSetupPlugin) // play scene management
        .add_plugins(home_set::HomeSetPlugin) // home scene setup
        .run();
}
