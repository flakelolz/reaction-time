use bevy::prelude::*;

use crate::AppState;

use super::score::Scores;

pub struct ResultPlugin;

impl Plugin for ResultPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_result_ui)
            .add_systems(OnEnter(AppState::Result), show_result_ui)
            .add_systems(Update, update_result_ui.run_if(in_state(AppState::Result)))
            .add_systems(OnExit(AppState::Result), hide_result_ui);
    }
}

#[derive(Component)]
struct ResultUI;

#[derive(Component)]
struct NormalText;

fn setup_result_ui(mut commands: Commands) {
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
        ResultUI,
    );

    let result = (
        TextBundle::from_section(
            "Result",
            TextStyle {
                font_size: 110.,
                color: Color::WHITE,
                ..default()
            },
        ),
        ResultUI,
    );

    let click = (
        TextBundle::from_section(
            "Click to continue",
            TextStyle {
                font_size: 50.,
                color: Color::WHITE,
                ..default()
            },
        ),
        NormalText,
    );

    let result = commands.spawn(result).id();
    let click = commands.spawn(click).id();

    commands.spawn(container).push_children(&[result, click]);
}

fn show_result_ui(mut text: Query<&mut Style, With<ResultUI>>) {
    for mut style in &mut text {
        style.display = Display::Flex;
    }
}

fn update_result_ui(mut text: Query<&mut Text, With<ResultUI>>, scores: Res<Scores>) {
    for mut text in &mut text {
        text.sections[0].value = format!(
            "{:.0?}",
            scores.reactions[scores.counter - 1].unwrap_or_default()
        );
    }
}

fn hide_result_ui(mut text: Query<&mut Style, Or<(With<ResultUI>, With<NormalText>)>>) {
    for mut style in &mut text {
        style.display = Display::None;
    }
}
