use bevy::{prelude::*, time::Stopwatch};
use std::time::Duration;

mod input;
mod reaction;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Reaction Test".to_string(),
                resolution: (800.0, 600.0).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(Scores::new())
        .init_state::<AppState>()
        .add_plugins(ui::InterfacePlugin)
        .add_plugins(input::InputPlugin)
        .add_plugins(reaction::ReactionPlugin)
        .add_systems(Startup, setup)
        .run();
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum AppState {
    #[default]
    Start,
    Playing,
    Result,
}

#[derive(Resource)]
struct Scores {
    pub counter: usize,
    pub size: usize,
    pub reactions: Vec<Option<Duration>>,
}

impl Scores {
    fn new() -> Self {
        Self {
            counter: 0,
            size: 5,
            reactions: vec![None; 5],
        }
    }

    fn average(&self) -> Option<Duration> {
        if self.counter == self.size {
            self.reactions
                .iter()
                .filter_map(|reaction| *reaction)
                .sum::<Duration>()
                .checked_div(self.reactions.len() as u32)
        } else {
            None
        }
    }

    fn reset(&mut self) {
        self.counter = 0;
        self.reactions = vec![None; self.size];
    }
}

#[derive(Resource)]
struct TimeTracking {
    countdown: Stopwatch,
    reaction_time: Timer,
}

impl TimeTracking {
    fn new() -> Self {
        Self {
            countdown: Stopwatch::new(),
            reaction_time: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
