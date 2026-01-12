use bevy::ecs::message::Message;

#[derive(Message, Clone, Copy)]
pub struct MoveEvent {
    pub dx: f32,
    pub dy: f32,
    pub dz: f32,
}
