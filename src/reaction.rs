use bevy::prelude::*;

use crate::AppState;

pub struct ReactionPlugin;

impl Plugin for ReactionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_colors)
            .add_systems(Update, square_color)
            .add_systems(Update, square_size);
    }
}

#[derive(Component)]
pub struct ReactionColor;

fn setup_colors(mut commands: Commands, window: Query<&Window>) {
    let window = window.single();
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(window.width(), window.height())),
                ..Default::default()
            },
            ..Default::default()
        },
        ReactionColor,
    ));
}

fn square_color(mut square: Query<&mut Sprite, With<ReactionColor>>, state: Res<State<AppState>>) {
    let state = state.as_ref().get();
    for mut sprite in &mut square {
        sprite.color = match state {
            AppState::Idle => Color::rgb_u8(43, 135, 209),
            AppState::Countdown => Color::rgb_u8(206, 38, 54),
            AppState::Misinput => Color::rgb_u8(43, 135, 209),
            AppState::Listening => Color::rgb_u8(75, 219, 106),
            AppState::Result => Color::rgb_u8(75, 219, 106),
            AppState::Finished => Color::BLACK,
        }
    }
}

fn square_size(mut square: Query<&mut Sprite, With<ReactionColor>>, window: Query<&Window>) {
    let window = window.single();

    for mut sprite in &mut square {
        sprite.custom_size = Some(Vec2::new(window.width(), window.height()));
    }
}
