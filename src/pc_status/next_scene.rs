use crate::GameState;
use bevy::prelude::*;

pub struct GoPlaySceneUIPlugin;

impl Plugin for GoPlaySceneUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::CalculatePC), setup_go_play_scene)
            .add_systems(
                Update,
                update_go_play_scene.run_if(in_state(GameState::CalculatePC)),
            );
    }
}

#[derive(Component)]
pub struct GoPlaySceneButton;

fn setup_go_play_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(go_play_scene_button(asset_server));
}

fn go_play_scene_button(asset_server: Res<AssetServer>) -> impl Bundle {
    (
        DespawnOnExit(GameState::CalculatePC),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Button,
            GoPlaySceneButton,
            Node {
                width: px(150),
                height: px(65),
                // Show below the center button
                margin: UiRect::top(px(100.0)),
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
                Text::new("Go to Play Scene"),
                TextFont {
                    font: asset_server.load("embedded://drone_race/fonts/NotoSansJP-Bold.ttf"),
                    font_size: 20.0,
                    ..default()
                },
                TextLayout::new_with_justify(Justify::Center),
            ),],
        ),],
    )
}

fn update_go_play_scene(
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<GoPlaySceneButton>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut background_color) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                background_color.0 = Color::srgb(0.5, 0.5, 0.5);
                game_state.set(GameState::PlayScene);
            }
            Interaction::Hovered => {
                background_color.0 = Color::srgb(0.7, 0.7, 0.7);
            }
            Interaction::None => {
                background_color.0 = Color::srgb(0.9, 0.9, 0.9);
            }
        }
    }
}
