use bevy::ecs::system::Commands;

use crate::components::{
    player::{Player, PlayerId},
    position::Position,
};

pub fn add_player(mut commands: Commands) {
    commands.spawn((Player, PlayerId(0), Position::new(10.0, 10.0)));
}
