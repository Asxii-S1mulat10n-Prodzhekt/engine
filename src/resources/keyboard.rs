use bevy::ecs::message::Message;

#[derive(Message, Clone, Copy, PartialEq, Eq, Debug)]
pub enum MovementMessage {
    Forward,
    Left,
    Right,
    Backward,
}
