use bevy::{prelude::*, time::Stopwatch};

mod input;
mod reaction;
mod score;

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
        .init_state::<AppState>()
        .add_plugins(score::InterfacePlugin)
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
