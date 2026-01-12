use crate::{
    components::{position::Position, server::ClientId},
    entities::Player,
    resources::{events::MoveEvent, player::OwnedBy},
};
use bevy::ecs::{
    message::MessageReader,
    system::{Query, Res},
};

pub fn movement_system(
    mut events: MessageReader<MoveEvent>,
    mut query: Query<(&Player, &ClientId, &mut Position)>,
    owned_id: Res<OwnedBy>,
) {
    for e in events.read() {
        for (_, id, mut pos) in query.iter_mut() {
            pos.gravity();
            if &owned_id.0 == id {
                pos.incress(e.dx, e.dy, e.dz);
            }
        }
    }
}
