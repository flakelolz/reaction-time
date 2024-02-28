use bevy::prelude::*;

use crate::ui::score::Scores;
use crate::AppState;

pub struct InputPlugin;

#[derive(Event)]
pub enum InputEvent {
    Click,
    Restart,
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputEvent>()
            .add_systems(Update, input)
            .add_systems(Update, reset);
    }
}

fn input(
    key: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut writer: EventWriter<InputEvent>,
) {
    if key.just_pressed(KeyCode::KeyJ) || mouse.just_pressed(MouseButton::Left) {
        writer.send(InputEvent::Click);
    }
}

fn reset(
    key: Res<ButtonInput<KeyCode>>,
    mut scores: ResMut<Scores>,
    states: Res<State<AppState>>,
    mut writer: EventWriter<InputEvent>,
) {
    if key.just_pressed(KeyCode::KeyR) && states.as_ref() != &AppState::Playing {
        writer.send(InputEvent::Restart);
        scores.reset();
    }
}
