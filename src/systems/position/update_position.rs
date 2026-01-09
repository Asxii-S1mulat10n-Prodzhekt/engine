use crate::{components::position::Position, entities::EntityId, resources::events::MoveEvent};
use bevy::ecs::{message::MessageReader, system::Query};

pub fn movement_system(
    mut events: MessageReader<MoveEvent>,
    mut query: Query<(&EntityId, &mut Position)>,
) {
    for e in events.read() {
        for (id, mut pos) in query.iter_mut() {
            if *id == e.entity {
                pos.incress(e.dx, e.dy);
            }
        }
    }
}
