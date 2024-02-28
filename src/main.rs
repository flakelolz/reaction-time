use bevy::prelude::*;

mod input;
mod reaction;
mod score;
mod countdown;
mod debug;
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
        .add_plugins(debug::DebugPlugin)
        .add_plugins(score::InterfacePlugin)
        .add_plugins(input::InputPlugin)
        .add_plugins(reaction::ReactionPlugin)
        .add_plugins(ui::instructions::InstructionsPlugin)
        .add_plugins(countdown::CountdownPlugin)
        .add_systems(Startup, setup)
        .run();
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum AppState {
    #[default]
    Start,
    Playing,
    Result,
}


fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
