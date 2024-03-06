use bevy::{prelude::*, time::Stopwatch};
use rand::Rng;

use crate::{ui::score::Scores, AppState};

pub struct StateMachinePlugin;

impl Plugin for StateMachinePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TimeKeeper::new(1.5))
            .add_systems(Update, app_logic);
    }
}

#[derive(Resource, Default)]
pub struct TimeKeeper {
    pub countdown: Timer,
    reaction: Stopwatch,
    transition: Timer,
}

impl TimeKeeper {
    fn new(seconds: f32) -> Self {
        Self {
            countdown: Timer::from_seconds(seconds, TimerMode::Once),
            transition: Timer::from_seconds(1.0, TimerMode::Once),
            reaction: Stopwatch::new(),
        }
    }

    fn reset(&mut self) {
        self.countdown.reset();
        self.reaction.reset();
        self.transition.reset();
    }
}

fn app_logic(
    state: ResMut<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut timers: ResMut<TimeKeeper>,
    mut scores: ResMut<Scores>,
    input: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
) {
    match state.get() {
        AppState::Idle => {
            scores.reset();
            timers.reset();
            // Click to restart the game
            if input.just_pressed(MouseButton::Left) {
                next_state.set(AppState::Countdown);
            }
        }
        // while playing
        AppState::Countdown => {
            // when countdown finished, it will either restart or transition to listening to
            // clicks
            if timers.countdown.finished() {
                match rand::thread_rng().gen_bool(0.5) {
                    true => {
                        // start a fresh timer for reaction calculation
                        timers.reaction.reset();
                        next_state.set(AppState::Listening)
                    }
                    false => {
                        timers.countdown.reset();
                        next_state.set(AppState::Countdown)
                    }
                }
            // else if you click to soon, it will transition to misinput
            } else if input.just_pressed(MouseButton::Left) {
                next_state.set(AppState::Misinput);
            }
        }
        // in case of misinput, click to go back to countdown
        AppState::Misinput => {
            if input.just_pressed(MouseButton::Left) {
                next_state.set(AppState::Countdown)
            }
        }
        AppState::Listening => {
            // Start tracking reaction time
            timers.reaction.tick(time.delta());

            // When clicked, add reaction time to scores
            if input.just_pressed(MouseButton::Left) {
                scores.add(timers.reaction.elapsed());

                // Show the current reaction time
                next_state.set(AppState::Result);
            }
        }
        AppState::Result => {
            if input.just_pressed(MouseButton::Left) {
                // if the reaction Vec is filled, show the finished state
                if scores.counter == scores.size {
                    next_state.set(AppState::Finished);
                } else {
                    next_state.set(AppState::Countdown);
                }
            }
        }
        AppState::Finished => {
            if input.just_pressed(MouseButton::Left) {
                next_state.set(AppState::Idle);
            }
        }
    }
}
