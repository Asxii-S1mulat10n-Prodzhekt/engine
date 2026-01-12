use bevy::ecs::component::Component;
#[derive(Component)]
#[component(storage = "Table")] // Default
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}
impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn incress(&mut self, dx: f32, dy: f32, dz: f32) {
        self.x = (self.x + dx).clamp(0.0, 50.0);
        self.y = (self.y + dy).clamp(0.0, 50.0);
        self.z = (self.z + dz).clamp(0.0, 50.0);
    }

    pub fn get_position_as_u16(&self) -> (u16, u16, u16) {
        (self.x as u16, self.y as u16, self.z as u16)
    }
}
impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Position(x: {}, y: {})", self.x, self.y)
    }
}
