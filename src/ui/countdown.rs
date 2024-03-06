use bevy::prelude::*;

use crate::{fsm::TimeKeeper, AppState};

pub struct CountdownPlugin;

impl Plugin for CountdownPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_countdown_ui)
            .add_systems(OnEnter(AppState::Countdown), show_countdown)
            .add_systems(
                Update,
                update_countdown.run_if(in_state(AppState::Countdown)),
            )
            .add_systems(OnExit(AppState::Countdown), hide_countdown);
    }
}

#[derive(Component)]
struct CountdownUI;

fn setup_countdown_ui(mut commands: Commands) {
    let container = (
        NodeBundle {
            style: Style {
                display: Display::None,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            ..default()
        },
        CountdownUI,
    );

    let counter = (
        TextBundle::from_section(
            "Countdown",
            TextStyle {
                font_size: 150.,
                color: Color::WHITE,
                ..default()
            },
        ),
        CountdownUI,
    );

    let counter = commands.spawn(counter).id();
    commands.spawn(container).push_children(&[counter]);
}

// Shows and ticks the countdown while on ReactionState::Countdown state
fn show_countdown(mut text_query: Query<&mut Style, With<CountdownUI>>) {
    for mut style in &mut text_query {
        style.display = Display::Flex;
    }
}

fn update_countdown(
    mut text: Query<&mut Text, With<CountdownUI>>,
    mut timer: ResMut<TimeKeeper>,
    time: Res<Time>,
) {
    timer.countdown.tick(time.delta());
    for mut text in &mut text {
        text.sections[0].value = format!("{:.1}", timer.countdown.remaining_secs());
    }
}

// Hides and resets the countdown on any other state
fn hide_countdown(
    mut text_query: Query<&mut Style, With<CountdownUI>>,
    mut timer: ResMut<TimeKeeper>,
) {
    timer.countdown.reset();

    for mut style in &mut text_query {
        style.display = Display::None;
    }
}
