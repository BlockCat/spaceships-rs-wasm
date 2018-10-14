use std::ops::{Add, Sub};
use std::convert::Into;
use quicksilver::geom::Vector;
use rand::Rng;
use geometry::Size;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32
}

impl Point {
    pub fn random<R: Rng>(rng: &mut R, bounds: &Size) -> Point {
        Point {
            x: rng.gen_range(0f32, bounds.width as f32),
            y: rng.gen_range(0f32, bounds.height as f32)
        }
    }

    pub fn distance_squared(&self, other: &Point) -> f32 {

        let mut dx1 = (self.x - other.x).abs();
        let mut dy1 = (self.y - other.y).abs();

        if dx1 > 400.0 {
            dx1 = 800.0 - dx1;
        }
        if dy1 > 300.0 {
            dy1 = 600.0 - dy1;
        }

        return dx1 * dx1 + dy1 * dy1;
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, _rhs: Point) -> Point {
        Point {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y
        }
    }
}

impl Add<f32> for Point {
    type Output = Point;

    fn add(self, _rhs: f32) -> Point {
        Point {
            x: self.x + _rhs,
            y: self.y + _rhs
        }
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, _rhs: Vector) -> Point {
        Point {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, _rhs: Point) -> Point {
        Point {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y
        }
    }
}

impl Sub<f32> for Point {
    type Output = Point;

    fn sub(self, _rhs: f32) -> Point {
        Point {
            x: self.x - _rhs,
            y: self.y - _rhs
        }
    }
}

impl Into<Vector> for Point {
    fn into(self) -> Vector {
        Vector {
            x: self.x,
            y: self.y
        }
    }
}