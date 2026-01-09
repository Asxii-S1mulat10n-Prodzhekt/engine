use bevy::prelude::*;
use bevy_ratatui::RatatuiPlugins;
use engine::{
    entities::EntityId,
    resources::{events::MoveEvent, player::LocalPlayer},
    systems::prelude::*,
};
fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins.set(bevy::app::ScheduleRunnerPlugin::run_loop(
                std::time::Duration::from_secs_f32(1. / 60.),
            )),
            RatatuiPlugins::default(),
        ))
        .insert_resource(LocalPlayer(EntityId(0)))
        .add_message::<MoveEvent>()
        .add_systems(Startup, add_player)
        .add_systems(PreUpdate, input_system)
        .add_systems(Update, movement_system)
        .add_systems(Update, draw_system)
        .run();
}
