use bevy::math::Vec3;
use bevy::prelude::*;
use ndarray::{Array3, s};

pub struct Quad {
    pub p1: Vec3,
    pub p2: Vec3,
    pub p3: Vec3,
    pub p4: Vec3,
}
#[derive(Resource)]
pub struct Obstacles {
    position: Array3<f32>, // (nth obstacle,nth section,(x,y,z))
    n_obstacles: usize,
    max_sections: usize,
}
impl Obstacles {
    pub fn new() -> Self {
        let vpos = vec![
            0.0, 0.0, 0.0, //
            0.0, 0.0, 50.0, //
            60.0, 0.0, 0.0, //
            60.0, 0.0, 50.0, //wall1
            0.0, 60.0, 0.0, //
            0.0, 60.0, 50.0, //
            60.0, 60.0, 0.0, //
            60.0, 60.0, 50.0, //wall1 //box
        ];

        let position = Array3::from_shape_vec((2, 4, 3), vpos).expect("Invalid shape");
        Self {
            position,
            n_obstacles: 2,
            max_sections: 4,
        }
    }
}
