use crate::Scores;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

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

            let count = times.counter;
            times.reactions[count] = Some(std::time::Duration::from_millis(100));
            times.counter += 1;
        }

        for (i, time) in times.reactions.iter().enumerate() {
            if let Some(time) = time {
                ui.label(format!("{}: {:?}", i + 1, time));
            } else {
                ui.label(format!("{}: None", i + 1));
            }
        }
    });

    egui::Window::new("Actions").show(contexts.ctx_mut(), |ui| {
        if ui.button("Size of 5").clicked() {
            times.size = 5;
            times.reset();
        }

        if ui.button("Size of 10").clicked() {
            times.size = 10;
            times.reset();
        }
    });
}
