use crate::components::server::ClientId;
use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub struct OwnedBy(pub ClientId);
