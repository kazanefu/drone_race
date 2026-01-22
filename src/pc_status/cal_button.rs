use crate::GameState;
use bevy::prelude::*;

pub struct CalButtonPlugin;

impl Plugin for CalButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::CalculatePC), setup_cal_button)
            .add_message::<CalPcStatusMessage>()
            .add_systems(
                Update,
                update_cal_button.run_if(in_state(GameState::CalculatePC)),
            );
    }
}

#[derive(Message)]
pub struct CalPcStatusMessage;

#[derive(Component)]
pub struct CalButton;

fn setup_cal_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(cal_button(asset_server));
}

fn cal_button(asset_server: Res<AssetServer>) -> impl Bundle {
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
            CalButton,
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
                Text::new("Calculate from PC status"),
                TextFont {
                    font: asset_server.load("fonts/NotoSansJP-Bold.ttf"),
                    font_size: 20.0,
                    ..default()
                },
                TextLayout::new_with_justify(Justify::Center),
            ),],
        ),],
    )
}

fn update_cal_button(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<CalButton>)>,
    mut cal_pc_status_event: MessageWriter<CalPcStatusMessage>,
) {
    for (interaction, mut background_color) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                background_color.0 = Color::srgb(0.5, 0.5, 0.5);
                cal_pc_status_event.write(CalPcStatusMessage);
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
