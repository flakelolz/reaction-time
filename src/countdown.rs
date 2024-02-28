use bevy::prelude::*;

pub struct CountdownPlugin;

impl Plugin for CountdownPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TimeKeeper::new(3.0))
            .add_systems(Update, countdown_ui_system);
    }
}

#[derive(Resource, Default)]
struct TimeKeeper {
    countdown: Timer,
}

impl TimeKeeper {
    fn new(seconds: f32) -> Self {
        Self {
            countdown: Timer::from_seconds(seconds, TimerMode::Once),
        }
    }
}

fn countdown_ui_system(mut timer: ResMut<TimeKeeper>, time: Res<Time>) {
    if !timer.countdown.tick(time.delta()).just_finished() {
        let count = timer.countdown.remaining_secs();
        // println!("{:.2?}", count);
    }
}
