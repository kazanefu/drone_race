mod game_rules;
mod home;
mod input;
mod pc_status;
mod playscene;

use bevy::prelude::*;
use game_rules::{GameRulesPlugin, GameState};
use input::InputPlugin;
use pc_status::pc_status::PcStatusPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    bevy::asset::embedded_asset!(app, "fonts/NotoSansJP-Bold.ttf");
    app.add_plugins(PcStatusPlugin) // pc status management
        .add_plugins(GameRulesPlugin) // game state management
        .add_plugins(InputPlugin)
        .add_plugins(playscene::playscene_setup::PlaySceneSetupPlugin) // play scene management
        .add_plugins(home::home_set::HomeSetPlugin) // home scene setup
        .run();
}
