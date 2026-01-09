use bevy::ecs::{component::Component, message::Message};
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerId(pub u32);
#[derive(Component)]
pub struct Player;

#[derive(Message, Clone, Copy)]
pub struct MoveEvent {
    pub player: PlayerId,
    pub dx: f32,
    pub dy: f32,
}
