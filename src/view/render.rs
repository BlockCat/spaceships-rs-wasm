use quicksilver::{
    geom::Circle,
    graphics::{Background::Col, Color},
    lifecycle::{Window},
};
use models::World;

pub struct Render {

}

impl Render {
    
    pub fn render_spaceships(world: &World, window: &mut Window) {
        for spaceship in world.spaceships.iter() {

            window.draw(&Circle::new(spaceship.location, 6), Col(Color::GREEN));
        }

    }

}