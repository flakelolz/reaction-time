use std::time::Duration;

use bevy::prelude::*;

use crate::AppState;

pub struct ScoresPlugin;

impl Plugin for ScoresPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Scores::new())
            .add_systems(Startup, setup_remaining_ui)
            .add_systems(Update, update_remaining_ui.before(hide_remaining_ui))
            .add_systems(
                Update,
                hide_remaining_ui
                    .run_if(in_state(AppState::Idle).or_else(in_state(AppState::Finished))),
            );
    }
}

#[derive(Component)]
struct ScoresUI;

#[derive(Resource)]
pub struct Scores {
    pub counter: usize,
    pub size: usize,
    pub reactions: Vec<Option<Duration>>,
}

impl Scores {
    fn new() -> Self {
        Self {
            counter: 0,
            size: 5,
            reactions: vec![None; 5],
        }
    }

    pub fn average(&self) -> Option<Duration> {
        if self.counter == self.size {
            self.reactions
                .iter()
                .filter_map(|reaction| *reaction)
                .sum::<Duration>()
                .checked_div(self.reactions.len() as u32)
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.counter = 0;
        self.reactions = vec![None; self.size];
    }

    pub fn add(&mut self, reaction: Duration) {
        self.reactions[self.counter] = Some(reaction);
        self.counter += 1;
    }
}

impl std::fmt::Display for Scores {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "1:     {:.0?}\n2:     {:.0?}\n3:     {:.0?}\n4:     {:.0?}\n5:     {:.0?}\n\nAvg: {:.0?}\n",
            self.reactions[0].unwrap_or_default(),
            self.reactions[1].unwrap_or_default(),
            self.reactions[2].unwrap_or_default(),
            self.reactions[3].unwrap_or_default(),
            self.reactions[4].unwrap_or_default(),
            self.average().unwrap_or_default()
        )
    }
}

fn setup_remaining_ui(mut commands: Commands) {
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

    let remaining = (
        TextBundle::from_section(
            "3/5",
            TextStyle {
                font_size: 45.5,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Percent(2.),
            right: Val::Percent(2.),
            ..default()
        }),
        ScoresUI,
    );

    let remaining = commands.spawn(remaining).id();
    commands.spawn(container).push_children(&[remaining]);
}

fn update_remaining_ui(
    mut query: Query<(&mut Text, &mut Style), With<ScoresUI>>,
    scores: Res<Scores>,
) {
    for (mut text, mut style) in &mut query {
        style.display = Display::Flex;
        text.sections[0].value = format!("{}/{}", scores.counter, scores.size);
    }
}

fn hide_remaining_ui(mut query: Query<&mut Style, With<ScoresUI>>) {
    for mut style in &mut query {
        style.display = Display::None;
    }
}
