use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2},
    EguiContexts, EguiPlugin,
};

use crate::AppState;
use rand::Rng;

use super::score::Scores;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Update, debug_window);
    }
}

fn debug_window(
    mut score: ResMut<Scores>,
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<AppState>>,
    state: Res<State<AppState>>,
) {
    egui::Window::new("Debug")
        .anchor(Align2::LEFT_TOP, (0., -10.))
        .default_open(true)
        .show(contexts.ctx_mut(), |ui| {
            // Reaction Times
            ui.heading("Times");
            for (i, time) in score.reactions.iter().enumerate() {
                if let Some(time) = time {
                    ui.label(format!("[#{}]: {:?}", i + 1, time));
                } else {
                    ui.label(format!("[#{}]: ", i + 1));
                }
            }

            if let Some(average) = score.average() {
                ui.label(format!("Avg: {:?}", average));
            } else {
                ui.label("Avg: Calculating...");
            }

            ui.horizontal(|ui| {
                if ui.button("Add").clicked() {
                    if score.counter == score.size {
                        score.reset();
                    }

                    let rng = rand::thread_rng().gen_range(0..100);
                    let count = score.counter;
                    score.reactions[count] =
                        Some(std::time::Duration::from_millis(200 + rng as u64));
                    score.counter += 1;
                }

                if ui.button("Reset").clicked() {
                    score.reset();
                    next_state.set(AppState::Idle);
                }
            });

            // App State
            ui.heading("States");
            ui.vertical(|ui| {
                if ui
                    .selectable_label(*state.get() == AppState::Idle, "Idle")
                    .clicked()
                {
                    next_state.set(AppState::Idle);
                }

                if ui
                    .selectable_label(*state.get() == AppState::Countdown, "Countdown")
                    .clicked()
                {
                    next_state.set(AppState::Countdown);
                }

                if ui
                    .selectable_label(*state.get() == AppState::Misinput, "Misinput")
                    .clicked()
                {
                    next_state.set(AppState::Misinput);
                }

                if ui
                    .selectable_label(*state.get() == AppState::Listening, "Listening")
                    .clicked()
                {
                    next_state.set(AppState::Listening);
                }

                if ui
                    .selectable_label(*state.get() == AppState::Result, "Result")
                    .clicked()
                {
                    next_state.set(AppState::Result);
                }

                if ui
                    .selectable_label(*state.get() == AppState::Finished, "Finished")
                    .clicked()
                {
                    next_state.set(AppState::Finished);
                }
            })
        });
}
