use crate::entities::EntityId;
use bevy::ecs::message::Message;

#[derive(Message, Clone, Copy)]
pub struct MoveEvent {
    pub entity: EntityId,
    pub dx: f32,
    pub dy: f32,
}
