use bevy::prelude::*;

pub struct ReactionPlugin;

impl Plugin for ReactionPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_systems(Startup, setup_colors)
            .add_systems(Update, square_color)
            .add_systems(Update, square_size);
    }
}

#[allow(unused)]
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Idle,
    Countdown,
    Misinput,
    Listening,
    Results,
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

fn square_color(
    mut square: Query<&mut Sprite, With<ReactionColor>>,
    state: Res<State<AppState>>,
) {
    let state = state.as_ref().get();
    for mut sprite in &mut square {
        sprite.color = match state {
            AppState::Idle => Color::rgb_u8(43, 135, 209),
            AppState::Countdown => Color::rgb_u8(206, 38, 54),
            AppState::Misinput => Color::rgb_u8(43, 135, 209),
            AppState::Listening => Color::rgb_u8(75, 219, 106),
            AppState::Results => Color::BLACK,
        }
    }
}

fn square_size(mut square: Query<&mut Sprite, With<ReactionColor>>, window: Query<&Window>) {
    let window = window.single();

    for mut sprite in &mut square {
        sprite.custom_size = Some(Vec2::new(window.width(), window.height()));
    }
}
