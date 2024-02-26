use bevy::prelude::*;
use std::time::Duration;

mod input;
mod reaction;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scores::new())
        .init_state::<AppState>()
        .add_plugins(ui::InterfacePlugin)
        .add_plugins(input::InputPlugin)
        .add_plugins(reaction::ReactionPlugin)
        .run();
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum AppState {
    #[default]
    Start,
    Playing,
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

    fn reset(&mut self) {
        self.counter = 0;
        self.reactions = vec![None; self.size];
    }
}
