use bevy::prelude::*;

use crate::reaction::ReactionState;

pub struct InstructionsPlugin;

impl Plugin for InstructionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_instructions_ui)
            .add_systems(
                Update,
                show_instructions.run_if(in_state(ReactionState::Idle)),
            )
            .add_systems(
                Update,
                hide_instructions.run_if(not(in_state(ReactionState::Idle))),
            );
    }
}

#[derive(Component)]
struct InstructionsUI;

fn setup_instructions_ui(mut commands: Commands) {
    let container = NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        },
        ..default()
    };

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
            "Press any Key to play...",
            TextStyle {
                font_size: 25.,
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
        TextBundle::from_section(" ", TextStyle { font_size: 40., ..default() }),
        InstructionsUI,
    );

    let parent = commands.spawn(container).id();
    let child1 = commands.spawn(instructions).id();
    let child2 = commands.spawn(any_key).id();
    let space = commands.spawn(spacer).id();
    commands
        .entity(parent)
        .push_children(&[child1, space, child2]);
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
