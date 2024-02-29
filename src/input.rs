use bevy::prelude::*;

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
    //NOTE: How to make all alpha keys work?
    if key.any_just_pressed([]) || mouse.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        writer.send(InputEvent::Click);
    }
}

fn reset(key: Res<ButtonInput<KeyCode>>, mut writer: EventWriter<InputEvent>) {
    if key.just_pressed(KeyCode::Escape) {
        writer.send(InputEvent::Restart);
    }
}
