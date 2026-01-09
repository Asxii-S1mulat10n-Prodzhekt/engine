use bevy::ecs::component::Component;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct EntityId(pub u32);
