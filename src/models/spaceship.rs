use quicksilver::geom::Vector;
use rand::Rng;
use rand::Rand;
use geometry::Point;
use geometry::Size;

#[derive(Copy, Clone)]
pub struct Spaceship {
    pub id: i32,
    pub location: Point,
    pub direction: Vector
}



impl Spaceship {
    pub fn random<R: Rng>(rng: &mut R, size: &Size, id: i32) -> Spaceship {
        Spaceship {
            id,
            location: Point::random(rng, size),
            direction: (Vector::rand(rng).normalize() - Vector {x : 0.5f32, y: 0.5f32} ).normalize()
        }
    }
}