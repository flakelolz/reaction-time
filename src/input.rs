use bevy::prelude::*;

use crate::AppState;

pub struct InputPlugin;

#[derive(Event)]
pub enum InputEvent {
    Click,
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
    //NOTE: How to make all alpha keys work?
    if key.any_just_pressed([]) || mouse.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        writer.send(InputEvent::Click);
    }
}

fn reset(key: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if key.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::Idle);
    }
}
