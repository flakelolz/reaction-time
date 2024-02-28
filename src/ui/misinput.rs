use bevy::prelude::*;

use crate::reaction::ReactionState;

pub struct MisinputPlugin;

impl Plugin for MisinputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_misinput_ui)
            .add_systems(
                Update,
                show_misinput_ui.run_if(in_state(ReactionState::Misinput)),
            )
            .add_systems(
                Update,
                hide_misinput_ui.run_if(not(in_state(ReactionState::Misinput))),
            );
    }
}

#[derive(Component)]
struct MisinputUI;

fn setup_misinput_ui(mut commands: Commands) {
    let container = NodeBundle {
        style: Style {
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        },
        ..default()
    };

    let click = (
        TextBundle::from_section(
            "Too Soon!",
            TextStyle {
                font_size: 110.,
                color: Color::WHITE,
                ..default()
            },
        ),
        MisinputUI,
    );

    let child = commands.spawn(click).id();
    commands.spawn(container).push_children(&[child]);
}

fn show_misinput_ui(mut query: Query<&mut Style, With<MisinputUI>>) {
    for mut style in &mut query {
        style.display = Display::Flex;
    }
}
fn hide_misinput_ui(mut query: Query<&mut Style, With<MisinputUI>>) {
    for mut style in &mut query {
        style.display = Display::None;
    }
}
