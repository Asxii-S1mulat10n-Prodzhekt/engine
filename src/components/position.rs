use bevy::ecs::component::Component;
#[derive(Component)]
#[component(storage = "Table")] // Default
pub struct Position {
    x: f32,
    y: f32,
}
impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Position(x: {}, y: {})", self.x, self.y)
    }
}
