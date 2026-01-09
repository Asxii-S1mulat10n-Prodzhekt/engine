use bevy::ecs::resource::Resource;

use crate::entities::EntityId;

#[derive(Resource)]
pub struct LocalPlayer(pub EntityId);
