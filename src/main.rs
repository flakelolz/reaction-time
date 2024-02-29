use bevy::prelude::*;

mod fsm;
mod input;
mod reaction;
mod ui;

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
        .add_plugins(ui::debug::DebugPlugin)
        .add_plugins(reaction::ReactionPlugin)
        .add_plugins(input::InputPlugin)
        .add_plugins(ui::score::InterfacePlugin)
        .add_plugins(ui::instructions::InstructionsPlugin)
        .add_plugins(ui::countdown::CountdownPlugin)
        .add_plugins(ui::listening::ListeningPlugin)
        .add_plugins(ui::misinput::MisinputPlugin)
        .add_plugins(ui::result::ResultPlugin)
        .add_plugins(ui::finished::FinishedPlugin)
        .add_plugins(fsm::StateMachinePlugin)
        .add_systems(Startup, setup)
        .run();
}

#[allow(unused)]
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Idle,
    Countdown,
    Misinput,
    Listening,
    Result,
    Finished,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
