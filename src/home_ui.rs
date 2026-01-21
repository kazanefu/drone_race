use crate::GameState;
use bevy::prelude::*;

pub struct HomeUIPlugin;

impl Plugin for HomeUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Home), setup_home_ui)
            .add_systems(Update, update_home_ui.run_if(in_state(GameState::Home)));
    }
}
#[derive(Component)]
pub struct HomeUI;
fn setup_home_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Text::new("Home"),
        TextFont {
            font: asset_server.load("fonts/NotoSansJP-Bold.ttf"),
            font_size: 40.0,
            ..default()
        },
        TextLayout::new_with_justify(Justify::Center),
        HomeUI,
    ));
    commands.spawn(go_play_scene_button(asset_server));
}

fn go_play_scene_button(asset_server: Res<AssetServer>) -> impl Bundle {
    (
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Button,
            Node {
                width: px(150),
                height: px(65),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                border_radius: BorderRadius::MAX,
                ..default()
            },
            BorderColor::all(Color::WHITE),
            BackgroundColor(Color::BLACK),
            children![(
                Text::new("Play Scene"),
                TextFont {
                    font: asset_server.load("fonts/NotoSansJP-Bold.ttf"),
                    font_size: 20.0,
                    ..default()
                },
                TextLayout::new_with_justify(Justify::Center),
            ),],
        )],
    )
}

fn update_home_ui(mut query: Query<&mut Text, With<HomeUI>>) {}
