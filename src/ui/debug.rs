use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{reaction::ReactionState, AppState};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, change_state);
    }
}

fn change_state(
    mut contexts: EguiContexts,
    mut next_app: ResMut<NextState<AppState>>,
    curr_app: Res<State<AppState>>,
    mut next_reaction: ResMut<NextState<ReactionState>>,
    curr_reaction: Res<State<ReactionState>>,
) {
    egui::Window::new("Debug").show(contexts.ctx_mut(), |ui| {
        ui.label("App States");
        ui.horizontal(|ui| {
            if ui.selectable_label(*curr_app.get() == AppState::Start, "Start").clicked() {
                next_app.set(AppState::Start);
            }

            if ui.selectable_label(*curr_app.get() == AppState::Playing, "Playing").clicked() {
                next_app.set(AppState::Playing);
            }

            if ui.selectable_label(*curr_app.get() == AppState::Result, "Result").clicked() {
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
                .selectable_label(*curr_reaction.get() == ReactionState::Countdown, "Countdown")
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
