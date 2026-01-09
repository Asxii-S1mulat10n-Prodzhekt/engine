use crate::components::{
    player::{MoveEvent, PlayerId},
    position::Position,
};
use bevy::ecs::{message::MessageReader, system::Query};

pub fn movement_system(
    mut events: MessageReader<MoveEvent>,
    mut query: Query<(&PlayerId, &mut Position)>,
) {
    for e in events.read() {
        for (id, mut pos) in query.iter_mut() {
            if *id == e.player {
                pos.incress(e.dx, e.dy);
            }
        }
    }
}
