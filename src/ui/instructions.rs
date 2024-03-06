use bevy::prelude::*;

use crate::AppState;

pub struct InstructionsPlugin;

impl Plugin for InstructionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_instructions_ui)
            .add_systems(OnEnter(AppState::Idle), show_instructions)
            .add_systems(OnExit(AppState::Idle), hide_instructions);
    }
}

#[derive(Component)]
struct InstructionsUI;

fn setup_instructions_ui(mut commands: Commands) {
    let container = (
        NodeBundle {
            style: Style {
                display: Display::None,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            ..default()
        },
        InstructionsUI,
    );

    let instructions = (
        TextBundle::from_section(
            "WAIT for the COUNTDOWN to END\nand REACT to the color CHANGE",
            TextStyle {
                font_size: 35.,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            display: Display::None,
            // position_type: PositionType::Absolute,
            ..default()
        }),
        InstructionsUI,
    );

    let any_key = (
        TextBundle::from_section(
            "Click to play...",
            TextStyle {
                font_size: 30.,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            display: Display::None,
            // position_type: PositionType::Absolute,
            ..default()
        }),
        InstructionsUI,
    );

    let spacer = (
        TextBundle::from_section(
            " ",
            TextStyle {
                font_size: 40.,
                ..default()
            },
        ),
        InstructionsUI,
    );

    let reset = (
        TextBundle::from_section(
            "Press ESC to Reset",
            TextStyle {
                font_size: 20.,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Percent(10.),
            ..default()
        }),
        InstructionsUI,
    );

    let instruction = commands.spawn(instructions).id();
    let any_key = commands.spawn(any_key).id();
    let spacer = commands.spawn(spacer).id();
    let reset = commands.spawn(reset).id();

    commands
        .spawn(container)
        .push_children(&[instruction, spacer, any_key, reset]);
}

fn show_instructions(mut instructions: Query<&mut Style, With<InstructionsUI>>) {
    for mut style in &mut instructions {
        style.display = Display::Flex;
    }
}

fn hide_instructions(mut instructions: Query<&mut Style, With<InstructionsUI>>) {
    for mut style in &mut instructions {
        style.display = Display::None;
    }
}
