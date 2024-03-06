use bevy::prelude::*;

use crate::AppState;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, reset);
    }
}

fn reset(key: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if key.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::Idle);
    }
}
