use std::ops::{Add, AddAssign, Mul};

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec2D {
    pub x: f64,
    pub y: f64,
}

impl Vec2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn rotate(&mut self, angle: f64) {
        let current_angle = self.y.atan2(self.x);

        let new_angle = current_angle + angle;
        let (dy, dx) = new_angle.sin_cos();
        let new_direction = Vec2D::new(dx, dy);

        let distance = self.x.hypot(self.y);

        *self = new_direction * distance;
    }

    pub fn manhattan_distance(&self) -> f64 {
        self.x.abs() + self.y.abs()
    }
}

impl Add<Self> for Vec2D {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign<Self> for Vec2D {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Mul<f64> for Vec2D {
    type Output = Self;

    fn mul(self, c: f64) -> Self::Output {
        Self {
            x: self.x * c,
            y: self.y * c,
        }
    }
}
