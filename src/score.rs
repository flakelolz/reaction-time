use std::time::Duration;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use rand::Rng;

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Scores::new())
            .add_plugins(EguiPlugin)
            .add_systems(Update, ui_system);
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

pub fn ui_system(mut contexts: EguiContexts, mut times: ResMut<Scores>) {
    egui::Window::new("Reaction Time").show(contexts.ctx_mut(), |ui| {
        if ui.button("Add").clicked() {
            if times.counter == times.size {
                times.reset();
            }

            let rng = rand::thread_rng().gen_range(0..100);
            let count = times.counter;
            times.reactions[count] = Some(std::time::Duration::from_millis(200 + rng as u64));
            times.counter += 1;
        }

        for (i, time) in times.reactions.iter().enumerate() {
            if let Some(time) = time {
                ui.label(format!("[#{}]: {:?}", i + 1, time));
            } else {
                ui.label(format!("[#{}]: ", i + 1));
            }
        }

        if let Some(average) = times.average() {
            ui.label(format!("Avg: {:?}", average));
        } else {
            ui.label("Avg: Calculating...");
        }

        if ui.button("Reset").clicked() {
            times.reset();
        }
    });
}
