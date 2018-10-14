use models::{Spaceship, Asteroid};
use std::vec::Vec;
use geometry::Size;

pub struct World {
    pub spaceships: Vec<Spaceship>,
    pub asteroids: Vec<Asteroid>
}

impl World {

    pub fn new(spaceships: i32, size: &Size) -> World {
        let mut rng = rand::thread_rng();

        let ship_vec = (0..spaceships).map(|i| Spaceship::random(&mut rng, size, i)).collect();



        return World {
            spaceships: ship_vec,
            asteroids: vec!()
        }
    }
}