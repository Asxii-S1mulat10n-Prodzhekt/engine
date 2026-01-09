use bevy::ecs::system::Commands;

use crate::components::{player::Player, position::Position};

pub fn add_players(mut commands: Commands) {
    commands.spawn((Player, Position::new(10.0, 10.0)));
}
