use bevy::prelude::*;

use crate::AppState;

use super::score::Scores;

pub struct ResultPlugin;

impl Plugin for ResultPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_result_ui)
            .add_systems(Update, show_result_ui.run_if(in_state(AppState::Result)))
            .add_systems(
                Update,
                hide_result_ui.run_if(not(in_state(AppState::Result))),
            );
    }
}

#[derive(Component)]
struct ResultUI;

#[derive(Component)]
struct NormalText;

fn setup_result_ui(mut commands: Commands) {
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
    commands
        .spawn(container)
        .push_children(&[result, click]);
}

fn show_result_ui(
    mut text: ParamSet<(
        Query<(&mut Text, &mut Style), With<ResultUI>>,
        Query<&mut Style, With<NormalText>>,
    )>,
    scores: Res<Scores>,
) {
    for (mut text, mut style) in &mut text.p0() {
        style.display = Display::Flex;
        text.sections[0].value = format!(
            "{:.0?}",
            scores.reactions[scores.counter - 1].unwrap_or_default()
        );
    }

    for mut style in &mut text.p1() {
        style.display = Display::Flex;
    }
}

fn hide_result_ui(mut text: Query<&mut Style, Or<(With<ResultUI>, With<NormalText>)>>) {
    for mut style in &mut text {
        style.display = Display::None;
    }
}
