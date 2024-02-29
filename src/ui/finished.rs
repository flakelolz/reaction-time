use bevy::prelude::*;

use crate::AppState;

use super::score::Scores;

pub struct FinishedPlugin;

impl Plugin for FinishedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_restart_ui)
            .add_systems(
                Update,
                (show_restart_ui, show_results).run_if(in_state(AppState::Finished)),
            )
            .add_systems(
                Update,
                hide_restart_ui.run_if(not(in_state(AppState::Finished))),
            );
    }
}

#[derive(Component)]
struct RestartUI;

#[derive(Component)]
struct ResultUI;

fn setup_restart_ui(mut commands: Commands) {
    let container = NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        },
        ..default()
    };

    let results = (
        TextBundle::from_section(
            "Reaction Times",
            TextStyle {
                font_size: 40.,
                color: Color::WHITE,
                ..default()
            },
        ),
        ResultUI,
        RestartUI,
    );

    let click = (
        TextBundle::from_section(
            "Press any Key to Restart...",
            TextStyle {
                font_size: 25.,
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
                font_size: 30.,
                color: Color::WHITE,
                ..default()
            },
        ),
        RestartUI,
    );

    let child1 = commands.spawn(results).id();
    let child2 = commands.spawn(click).id();
    let space = commands.spawn(spacer).id();
    commands
        .spawn(container)
        .push_children(&[child1, space, child2]);
}

fn show_results(mut query: Query<&mut Text, With<ResultUI>>, scores: Res<Scores>) {
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
