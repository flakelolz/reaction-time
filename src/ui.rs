use crate::Scores;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use rand::Rng;

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin).add_systems(Update, ui_system);
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
                ui.label(format!("{}: {:?}", i + 1, time));
            } else {
                ui.label(format!("{}: ...", i + 1));
            }
        }

        if let Some(average) = times.average() {
            ui.label(format!("Average: {:?}", average));
        } else {
            ui.label("Average: Calculating...");
        }

        if ui.button("Reset").clicked() {
            times.reset();
        }
    });
}
