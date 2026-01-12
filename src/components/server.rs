use bevy::ecs::component::Component;

#[derive(Component, PartialEq)]
pub struct ClientId(pub u64);
