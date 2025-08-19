use bevy::prelude::*;

#[derive(Resource)]
struct TickTimer(Timer);

#[derive(Resource, Default)]
struct ProcessState {
    counter: usize,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ProcessState::default())
        .insert_resource(TickTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_systems(Update, (tick_system, print_time_system))
        .run();
}

fn tick_system(
    time: Res<Time>,
    mut timer: ResMut<TickTimer>,
    mut state: ResMut<ProcessState>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        state.counter += 1;
        println!("Tick {} happened!", state.counter);
    }
}

fn print_time_system(time: Res<Time>) {
    println!(
        "Elapsed {:.2?}, Delta {:.4?}",
        time.elapsed(),
        time.delta()
    );
}
