use bevy::prelude::*;

pub struct ReactionPlugin;

impl Plugin for ReactionPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ReactionState>()
            .add_systems(Startup, spawn_square)
            .add_systems(Update, square_color)
            .add_systems(Update, square_size);
    }
}

#[allow(unused)]
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum ReactionState {
    #[default]
    Idle,
    Count,
    Misinput,
    Listening,
    Restart,
}

#[derive(Component)]
pub struct Square;

fn spawn_square(mut commands: Commands, window: Query<&Window>) {
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
        Square,
    ));
}

fn square_color(mut square: Query<&mut Sprite, With<Square>>, state: Res<State<ReactionState>>) {
    let state = state.as_ref().get();
    for mut sprite in &mut square {
        sprite.color = match state {
            ReactionState::Idle => Color::BLACK,
            ReactionState::Count => Color::RED,
            ReactionState::Misinput => Color::ALICE_BLUE,
            ReactionState::Listening => Color::GREEN,
            ReactionState::Restart => Color::BLUE,
        }
    }
}

fn square_size(mut square: Query<&mut Sprite, With<Square>>, window: Query<&Window>) {
    let window = window.single();

    for mut sprite in &mut square {
        sprite.custom_size = Some(Vec2::new(window.width(), window.height()));
    }
}
