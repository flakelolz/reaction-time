use bevy::prelude::*;

use crate::AppState;

pub struct ListeningPlugin;

impl Plugin for ListeningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_listening_ui)
            .add_systems(OnEnter(AppState::Listening), show_listening_ui)
            .add_systems(OnExit(AppState::Listening), hide_listening_ui);
    }
}

#[derive(Component)]
struct ListeningUI;

fn setup_listening_ui(mut commands: Commands) {
    let container = (
        NodeBundle {
            style: Style {
                display: Display::None,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            ..default()
        },
        ListeningUI,
    );

    let click = (
        TextBundle::from_section(
            "Click!",
            TextStyle {
                font_size: 110.,
                color: Color::WHITE,
                ..default()
            },
        ),
        ListeningUI,
    );

    let child = commands.spawn(click).id();
    commands.spawn(container).push_children(&[child]);
}

fn show_listening_ui(mut query: Query<&mut Style, With<ListeningUI>>) {
    for mut style in &mut query {
        style.display = Display::Flex;
    }
}

fn hide_listening_ui(mut query: Query<&mut Style, With<ListeningUI>>) {
    for mut style in &mut query {
        style.display = Display::None;
    }
}
