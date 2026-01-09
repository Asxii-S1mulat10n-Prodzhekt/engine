use bevy::ecs::system::Commands;

use crate::{
    components::{player::Player, position::Position},
    entities::EntityId,
};

pub fn add_player(mut commands: Commands) {
    commands.spawn((Player, EntityId(0), Position::new(10.0, 10.0)));
}
