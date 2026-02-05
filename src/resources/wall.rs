use bevy::math::Vec3;
#[derive(Clone, Copy)]
pub enum WallType {}
pub enum WallSide {
    Parallel,
    Collinear,
    Intersection { intersection: f32, numerator: f32 },
    OnFront,
    OnBack,
}

#[derive(Clone, Copy)]
pub struct Wall {
    start_botton_point: Vec3,
    end_botton_point: Vec3,
    start_top_point: Vec3,
    end_top_point: Vec3,
    walltype: WallType,
}
impl Wall {
    fn new(
        start_botton_point: Vec3,
        end_botton_point: Vec3,
        start_top_point: Vec3,
        end_top_point: Vec3,
        walltype: WallType,
    ) -> Self {
        Self {
            start_botton_point,
            end_botton_point,
            start_top_point,
            end_top_point,
            walltype,
        }
    }
    pub fn split(&self, intersection: f32, numerator: f32) -> (Self, Self) {
        let botton_intersection_point = self.start_botton_point + intersection * self.as_vector();
        let top_intersection_point = self.start_top_point + intersection * self.as_vector();
        let front = Self {
            start_botton_point: self.start_botton_point,
            end_botton_point: botton_intersection_point,
            start_top_point: self.start_top_point,
            end_top_point: top_intersection_point,
            walltype: self.walltype,
        };

        let back = Self {
            start_botton_point: botton_intersection_point,
            end_botton_point: self.end_botton_point,
            start_top_point: top_intersection_point,
            end_top_point: self.end_top_point,
            walltype: self.walltype,
        };
        if numerator > 0.0 {
            return (back, front);
        } else {
            return (front, back);
        }
    }
    fn cross_2d(lhs: Vec3, rhs: Vec3) -> f32 {
        lhs.x * rhs.y - rhs.x * lhs.y
    }

    pub fn as_vector(&self) -> Vec3 {
        self.end_botton_point - self.start_botton_point
    }
    pub fn splitter(&self, other_wall: &Wall) -> WallSide {
        let a = self.start_botton_point;
        let c = other_wall.start_botton_point;
        let numerator = Self::cross_2d(a - c, self.as_vector());
        let denominator = Self::cross_2d(self.as_vector(), other_wall.as_vector());
        let numerator_is_zero = numerator.abs() < f32::EPSILON;
        let denominator_is_zero = denominator.abs() < f32::EPSILON;
        if numerator_is_zero && denominator_is_zero {
            return WallSide::Collinear;
        }
        if !denominator_is_zero {
            let intersection = numerator / denominator;
            if (0.0..1.0).contains(&intersection) {
                return WallSide::Intersection {
                    intersection,
                    numerator,
                };
            }
        }
        if numerator < 0.0 || (numerator_is_zero && denominator > 0.0) {
            return WallSide::OnFront;
        }

        if numerator > 0.0 || (numerator_is_zero && denominator < 0.0) {
            return WallSide::OnBack;
        }
        WallSide::Parallel
    }
}
