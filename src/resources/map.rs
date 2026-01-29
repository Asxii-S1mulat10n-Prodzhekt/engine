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
            60.0, 60.0, 50.0, //wall1
        ];
        let position = Array3::from_shape_vec((2, 4, 3), vpos).expect("Invalid shape");
        Self {
            position,
            n_obstacles: 2,
            max_sections: 4,
        }
    }
    pub fn sort_by_distance(&self, pos: Vec3) -> Vec<((usize, usize), f32)> {
        let dx = &self.position.slice(s![.., .., 0]) - pos.x;
        let dy = &self.position.slice(s![.., .., 1]) - pos.y;
        let dz = &self.position.slice(s![.., .., 2]) - pos.z;
        let dist2 = &dx * &dx + &dy * &dy + &dz * &dz;
        let mut indexed: Vec<((usize, usize), f32)> = dist2
            .iter()
            .enumerate()
            .map(|(i, &d)| {
                let obstacle = i / self.max_sections;
                let section = i % self.max_sections;
                ((obstacle, section), d)
            })
            .collect();

        indexed.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        indexed
    }
    pub fn get_section(&self, obstacle: usize, pos: Vec3) -> Quad {
        Quad {
            p1: Vec3::from_slice(self.position.slice(s![obstacle, 0, ..]).as_slice().unwrap())
                - pos,
            p2: Vec3::from_slice(self.position.slice(s![obstacle, 1, ..]).as_slice().unwrap())
                - pos,
            p3: Vec3::from_slice(self.position.slice(s![obstacle, 2, ..]).as_slice().unwrap())
                - pos,
            p4: Vec3::from_slice(self.position.slice(s![obstacle, 3, ..]).as_slice().unwrap())
                - pos,
        }
    }

    pub fn get_positions(&self) -> &Array3<f32> {
        &self.position
    }
}
