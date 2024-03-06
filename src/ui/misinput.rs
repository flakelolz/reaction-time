use bevy::prelude::*;

use crate::AppState;

pub struct MisinputPlugin;

impl Plugin for MisinputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_misinput_ui)
            .add_systems(OnEnter(AppState::Misinput), show_misinput_ui)
            .add_systems(OnExit(AppState::Misinput), hide_misinput_ui);
    }
}

#[derive(Component)]
struct MisinputUI;

fn setup_misinput_ui(mut commands: Commands) {
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
        MisinputUI,
    );

    let soon = (
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

    let click = (
        TextBundle::from_section(
            "Click to try again",
            TextStyle {
                font_size: 50.,
                color: Color::WHITE,
                ..default()
            },
        ),
        MisinputUI,
    );

    let soon = commands.spawn(soon).id();
    let click = commands.spawn(click).id();
    commands.spawn(container).push_children(&[soon, click]);
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
