use bevy::prelude::*;
use bevy_ratatui::RatatuiPlugins;
use engine::systems::prelude::*;
fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins.set(bevy::app::ScheduleRunnerPlugin::run_loop(
                std::time::Duration::from_secs_f32(1. / 60.),
            )),
            RatatuiPlugins::default(),
        ))
        .add_systems(PreUpdate, input_system)
        .add_systems(Update, draw_system)
        .run();
}
