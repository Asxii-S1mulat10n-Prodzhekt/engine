use crate::components::player::PlayerId;
use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub struct LocalPlayer(pub PlayerId);
