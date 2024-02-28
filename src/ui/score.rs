use std::time::Duration;

use bevy::prelude::*;

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Scores::new());
    }
}

#[derive(Resource)]
pub struct Scores {
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

    pub fn average(&self) -> Option<Duration> {
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

    pub fn reset(&mut self) {
        self.counter = 0;
        self.reactions = vec![None; self.size];
    }
}
