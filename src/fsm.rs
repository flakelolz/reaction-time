use bevy::{prelude::*, time::Stopwatch};
use rand::Rng;

use crate::{input::InputEvent, reaction::ReactionState, ui::score::Scores, AppState};

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
    mut inputs: EventReader<InputEvent>,
    app: ResMut<State<AppState>>,
    mut next_app: ResMut<NextState<AppState>>,
    reaction: ResMut<State<ReactionState>>,
    mut next_reaction: ResMut<NextState<ReactionState>>,
    mut timers: ResMut<TimeKeeper>,
    mut scores: ResMut<Scores>,
    time: Res<Time>,
) {
    // Start the app
    match app.get() {
        AppState::Start => {
            // Click to start playing
            if let Some(InputEvent::Click) = inputs.read().last() {
                next_app.set(AppState::Playing);
                next_reaction.set(ReactionState::Countdown);
            }
        }
        AppState::Playing => match reaction.get() {
            ReactionState::Idle => {}
            // while playing
            ReactionState::Countdown => {
                // when countdown finished, it will either restart or transition to listening to
                // clicks
                if timers.countdown.finished() {
                    match rand::thread_rng().gen_bool(0.5) {
                        true => {
                            // start a fresh timer for reaction calculation
                            timers.reaction.reset();
                            next_reaction.set(ReactionState::Listening)
                        },
                        false => {
                            timers.countdown.reset();
                            next_reaction.set(ReactionState::Countdown)
                        }
                    }
                // else if you click to soon, it will transition to misinput
                } else if let Some(InputEvent::Click) = inputs.read().last() {
                    next_reaction.set(ReactionState::Misinput);
                }
            }
            // in case of misinput, click to go back to countdown
            ReactionState::Misinput => {
                if let Some(InputEvent::Click) = inputs.read().last() {
                    next_reaction.set(ReactionState::Countdown)
                }
            }
            ReactionState::Listening => {
                // Start tracking reaction time
                timers.reaction.tick(time.delta());
                // When clicked, add reaction time to scores
                if let Some(InputEvent::Click) = inputs.read().last() {
                    scores.add(timers.reaction.elapsed());
                }

                // When the reaction Vec is filled, show the results
                if scores.counter == scores.size {
                    next_reaction.set(ReactionState::Restart);
                }

                // wait the transition time to go back to countdown
                if timers.transition.tick(time.delta()).finished() {
                    timers.transition.reset();
                    next_reaction.set(ReactionState::Countdown);
                }
            }
            ReactionState::Restart => {
                if let Some(InputEvent::Click) = inputs.read().last() {
                    scores.reset();
                    timers.reset();
                    next_reaction.set(ReactionState::Idle);
                }
            }
        },
        AppState::Result => {
            if let Some(InputEvent::Click) = inputs.read().last() {
                next_app.set(AppState::Start);
            }
        }
    }
}
