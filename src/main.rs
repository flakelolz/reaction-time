#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::{asset::load_internal_binary_asset, prelude::*, window::WindowResolution};

mod fsm;
mod input;
mod reaction;
mod ui;

fn main() {
    let mut app = App::new();

    // Add everything to the app
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Reaction Test".to_string(),
            resolution: WindowResolution::new(800.0, 600.0).with_scale_factor_override(1.0),
            ..default()
        }),
        ..default()
    }))
    .init_state::<AppState>()
    .add_plugins(reaction::ReactionPlugin)
    .add_plugins(input::InputPlugin)
    .add_plugins(ui::score::ScoresPlugin)
    .add_plugins(ui::instructions::InstructionsPlugin)
    .add_plugins(ui::countdown::CountdownPlugin)
    .add_plugins(ui::listening::ListeningPlugin)
    .add_plugins(ui::misinput::MisinputPlugin)
    .add_plugins(ui::result::ResultPlugin)
    .add_plugins(ui::finished::FinishedPlugin)
    .add_plugins(fsm::StateMachinePlugin)
    .add_systems(Startup, setup)
    .add_plugins(ui::debug::DebugPlugin);

    // Change the default font
    load_internal_binary_asset!(
        app,
        TextStyle::default().font,
        "../assets/fonts/Helvetica Regular.otf",
        |bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
    );

    app.run();
}

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
