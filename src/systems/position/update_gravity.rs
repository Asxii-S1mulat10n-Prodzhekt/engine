use crate::{
    components::{position::Position, server::ClientId},
    entities::Player,
    resources::{events::MoveEvent, player::OwnedBy},
};
use bevy::ecs::{
    message::MessageReader,
    system::{Query, Res},
};

pub fn gravity_system(mut query: Query<(&Player, &mut Position)>) {
    for (_, mut pos) in query.iter_mut() {
        pos.gravity();
    }
}
