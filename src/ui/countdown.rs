use bevy::prelude::*;

use crate::reaction::ReactionState;

pub struct CountdownPlugin;

impl Plugin for CountdownPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TimeKeeper::new(1.5))
            .add_systems(Startup, setup_countdown_ui)
            .add_systems(
                Update,
                show_countdown.run_if(in_state(ReactionState::Countdown)),
            )
            .add_systems(
                Update,
                hide_countdown.run_if(not(in_state(ReactionState::Countdown))),
            );
    }
}

#[derive(Resource, Default)]
struct TimeKeeper {
    countdown: Timer,
}

impl TimeKeeper {
    fn new(seconds: f32) -> Self {
        Self {
            countdown: Timer::from_seconds(seconds, TimerMode::Once),
        }
    }
}

#[derive(Component)]
struct CountdownUI;

fn setup_countdown_ui(mut commands: Commands) {
    let container = NodeBundle {
        style: Style {
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        },
        ..default()
    };

    let count = (
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

    let parent = commands.spawn(container).id();
    let child = commands.spawn(count).id();
    commands.entity(parent).push_children(&[child]);
}

// Shows and ticks the countdown while on ReactionState::Countdown state
fn show_countdown(
    mut text_query: Query<(&mut Text, &mut Style), With<CountdownUI>>,
    mut timer: ResMut<TimeKeeper>,
    time: Res<Time>,
) {
    timer.countdown.tick(time.delta());

    for (mut text, mut style) in text_query.iter_mut() {
        style.display = Display::Flex;
        text.sections[0].value = format!("{:.2}", timer.countdown.remaining_secs());
    }
}

// Hides and resets the countdown on any other state
fn hide_countdown(
    mut text_query: Query<&mut Style, With<CountdownUI>>,
    mut timer: ResMut<TimeKeeper>,
) {
    timer.countdown.reset();

    for mut style in text_query.iter_mut() {
        style.display = Display::None;
    }
}
