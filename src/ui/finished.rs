use bevy::prelude::*;

use crate::AppState;

use super::score::Scores;

pub struct FinishedPlugin;

impl Plugin for FinishedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_restart_ui)
            .add_systems(OnEnter(AppState::Finished), (show_restart_ui, show_results))
            .add_systems(OnExit(AppState::Finished), hide_restart_ui);
    }
}

#[derive(Component)]
struct RestartUI;

#[derive(Component)]
struct ResultsTable;

fn setup_restart_ui(mut commands: Commands) {
    let container = (
        NodeBundle {
            style: Style {
                display: Display::None,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            ..default()
        },
        RestartUI,
    );

    let results = (
        TextBundle::from_section(
            "Reaction Times",
            TextStyle {
                font_size: 50.,
                color: Color::WHITE,
                ..default()
            },
        ),
        ResultsTable,
        RestartUI,
    );

    let click = (
        TextBundle::from_section(
            "Click to Restart...",
            TextStyle {
                font_size: 30.,
                color: Color::WHITE,
                ..default()
            },
        ),
        RestartUI,
    );

    let spacer = (
        TextBundle::from_section(
            " ",
            TextStyle {
                font_size: 40.,
                color: Color::WHITE,
                ..default()
            },
        ),
        RestartUI,
    );

    let results = commands.spawn(results).id();
    let click = commands.spawn(click).id();
    let spacer = commands.spawn(spacer).id();

    commands
        .spawn(container)
        .push_children(&[results, spacer, click]);
}

fn show_results(mut query: Query<&mut Text, With<ResultsTable>>, scores: Res<Scores>) {
    for mut text in &mut query {
        text.sections[0].value = format!("{}", scores.as_ref());
    }
}

fn show_restart_ui(mut query: Query<&mut Style, With<RestartUI>>) {
    for mut style in &mut query {
        style.display = Display::Flex;
    }
}
fn hide_restart_ui(mut query: Query<&mut Style, With<RestartUI>>) {
    for mut style in &mut query {
        style.display = Display::None;
    }
}
