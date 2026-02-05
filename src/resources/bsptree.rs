use crate::resources::wall::{Wall, WallSide};
use rand::seq::SliceRandom;
use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::SeedableRng;
use std::sync::{Arc, Mutex};
#[derive(Default)]
struct Score {
    pub front: i32,
    pub back: i32,
    pub split: i32,
}
impl Score {
    pub fn cal(&self, weight: i32) -> i32 {
        i32::abs(self.front - self.back) + (self.split * weight)
    }
}
pub(crate) enum BSPTree {
    Leaf,
    Node {
        value: Wall,
        front: Box<BSPTree>,
        back: Box<BSPTree>,
    },
}
impl BSPTree {
    pub fn new(walls: Vec<Wall>) -> Self {
        Self::build(walls)
    }
    pub fn build_with_seed(walls: Vec<Wall>, seed: u64) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut walls = walls;
        walls.shuffle(&mut rng);
        Self::build(walls)
    }
    pub fn find_seed(walls: &[Wall], iterations: u64, weight: i32) -> (u64, i32) {
        (0..iterations)
            .map(|seed| {
                let mut rng = ChaCha8Rng::seed_from_u64(seed);
                let mut shuffled = walls.to_vec();
                shuffled.shuffle(&mut rng);
                let score = Self::eval(shuffled).cal(weight);
                (seed, score)
            })
            .min_by_key(|&(_, score)| score)
            .unwrap_or((0, 0))
    }
    fn build(mut walls: Vec<Wall>) -> Self {
        if walls.is_empty() {
            return BSPTree::Leaf;
        }
        let mut front = Vec::with_capacity(walls.len());
        let mut back = Vec::with_capacity(walls.len());

        let splitter = walls.swap_remove(0);
        for wall in walls {
            match splitter.splitter(&wall) {
                WallSide::Collinear | WallSide::OnFront => {
                    front.push(wall);
                }
                WallSide::OnBack => {
                    back.push(wall);
                }
                WallSide::Intersection {
                    intersection,
                    numerator,
                } => {
                    let (front_wall, back_wall) = wall.split(intersection, numerator);
                    front.push(front_wall);
                    back.push(back_wall);
                }
                WallSide::Parallel => {}
            }
        }
        Self::Node {
            value: splitter,
            front: Box::new(Self::build(front)),
            back: Box::new(Self::build(back)),
        }
    }
    fn eval(mut walls: Vec<Wall>) -> Score {
        if walls.is_empty() {
            return Score::default();
        }

        let splitter = walls.swap_remove(0);

        let mut front = Vec::new();
        let mut back = Vec::new();

        let mut score = Score::default();

        for wall in walls {
            match splitter.splitter(&wall) {
                WallSide::Collinear | WallSide::OnFront => {
                    front.push(wall);
                    score.front += 1;
                }
                WallSide::OnBack => {
                    back.push(wall);
                    score.back += 1;
                }
                WallSide::Intersection {
                    intersection,
                    numerator,
                } => {
                    let (f, b) = wall.split(intersection, numerator);
                    front.push(f);
                    back.push(b);
                    score.split += 1;
                }
                WallSide::Parallel => {}
            }
        }

        let front_score = Self::eval(front);
        let back_score = Self::eval(back);

        score.front += front_score.front + back_score.front;
        score.back += front_score.back + back_score.back;
        score.split += front_score.split + back_score.split;

        score
    }
}
