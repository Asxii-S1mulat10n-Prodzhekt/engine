use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Player {
    px: f32,
    py: f32,
    angle_x: f32,
    angle_y: f32,
}
