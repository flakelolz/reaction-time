use bevy::prelude::*;
use bevy_egui::{egui::{self, Align2}, EguiContexts, EguiPlugin};

use crate::{reaction::ReactionState, AppState};
use rand::Rng;

use super::score::Scores;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Update, show_reactions)
            .add_systems(Update, change_state);
    }
}

pub fn show_reactions(mut contexts: EguiContexts, mut times: ResMut<Scores>) {
    egui::Window::new("Reaction Time")
        .anchor(Align2::LEFT_TOP, (0., 10.))
        .default_open(false)
        .show(contexts.ctx_mut(), |ui| {
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

fn change_state(
    mut contexts: EguiContexts,
    mut next_app: ResMut<NextState<AppState>>,
    curr_app: Res<State<AppState>>,
    mut next_reaction: ResMut<NextState<ReactionState>>,
    curr_reaction: Res<State<ReactionState>>,
) {
    egui::Window::new("Debug")
        .anchor(Align2::LEFT_BOTTOM, (0., -10.))
        .default_open(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.label("App States");
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(*curr_app.get() == AppState::Start, "Start")
                    .clicked()
                {
                    next_app.set(AppState::Start);
                }

                if ui
                    .selectable_label(*curr_app.get() == AppState::Playing, "Playing")
                    .clicked()
                {
                    next_app.set(AppState::Playing);
                }

                if ui
                    .selectable_label(*curr_app.get() == AppState::Result, "Result")
                    .clicked()
                {
                    next_app.set(AppState::Result);
                }
            });

            ui.label("Reaction State: ");
            ui.vertical(|ui| {
                if ui
                    .selectable_label(*curr_reaction.get() == ReactionState::Idle, "Idle")
                    .clicked()
                {
                    next_reaction.set(ReactionState::Idle);
                }

                if ui
                    .selectable_label(
                        *curr_reaction.get() == ReactionState::Countdown,
                        "Countdown",
                    )
                    .clicked()
                {
                    next_reaction.set(ReactionState::Countdown);
                }

                if ui
                    .selectable_label(*curr_reaction.get() == ReactionState::Misinput, "Misinput")
                    .clicked()
                {
                    next_reaction.set(ReactionState::Misinput);
                }

                if ui
                    .selectable_label(
                        *curr_reaction.get() == ReactionState::Listening,
                        "Listening",
                    )
                    .clicked()
                {
                    next_reaction.set(ReactionState::Listening);
                }

                if ui
                    .selectable_label(*curr_reaction.get() == ReactionState::Restart, "Restart")
                    .clicked()
                {
                    next_reaction.set(ReactionState::Restart);
                }
            })
        });
}
