use bevy::ecs::system::Commands;

use crate::{
    components::{position::Position, server::ClientId},
    entities::Player,
};

pub fn add_player(mut commands: Commands) {
    commands.spawn((Player, ClientId(0), Position::new(10.0, 10.0)));
}
