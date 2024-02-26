use bevy::prelude::*;

pub struct ReactionPlugin;

impl Plugin for ReactionPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ReactionState>();
    }
}


#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum ReactionState {
    #[default]
    Idle,
    Count,
    Waiting,
    Listening,
    Misinput,
    Results,
}

