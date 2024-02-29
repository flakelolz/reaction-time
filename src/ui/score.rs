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

    pub fn add(&mut self, reaction: Duration) {
        self.reactions[self.counter] = Some(reaction);
        self.counter += 1;
    }
}

impl std::fmt::Display for Scores {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "1:     {:.0?}\n2:     {:.0?}\n3:     {:.0?}\n4:     {:.0?}\n5:     {:.0?}\n\nAvg: {:.0?}\n",
            self.reactions[0].unwrap_or_default(),
            self.reactions[1].unwrap_or_default(),
            self.reactions[2].unwrap_or_default(),
            self.reactions[3].unwrap_or_default(),
            self.reactions[4].unwrap_or_default(),
            self.average().unwrap_or_default()
        )
    }
}
