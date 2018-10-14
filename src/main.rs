// Draw some multi-colored geometry to the screen
extern crate quicksilver;
extern crate rand;

mod geometry;
mod models;
mod view;

use quicksilver::{
    Result,
    geom::Vector,
    graphics::Color,
    lifecycle::{Settings, State, Window, run},
};
use models::World;
use view::Render;
use geometry::Size;
use models::Spaceship;

struct GameState {
    world: World
}



impl State for GameState {
    fn new() -> Result<GameState> {
        Ok(GameState {
            world: World::new(300, &Size {
                width: 800,
                height: 600
            })
        })
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {

        for i in 0..self.world.spaceships.len() {
            let direction = self.world.spaceships[i].direction;

            let mut alignment = Vector { x: 0.00, y: 0.00};
            let mut cohesion = Vector { x: 0.00, y: 0.00};
            let mut separation = Vector { x: 0.00, y: 0.0};
            {


                let mut space_ship = &self.world.spaceships[i];

                // To wrap around
                fn wrapVector(m: Vector) -> Vector {
                    let dx = if m.x < 400.0 {
                        m.x + 800.0f32
                    } else {
                        m.x
                    };

                    let dy= if m.y < 300.0 {
                        600.0f32 + m.y
                    } else {
                        m.y
                    };

                    Vector {
                        x: dx,
                        y: dy
                    }
                }

                // Find all the neighbours
                let neighbours: Vec<Spaceship> = self.world.spaceships.iter()
                    .filter(|i| {
                        let c = space_ship.location.distance_squared(&i.location);

                        i.id != space_ship.id && c < 500.0
                    })
                    .map(|i| i.clone())
                    .collect();

                if neighbours.len() > 0 {
                    // Alignment
                    for n in neighbours.iter() {
                        alignment = alignment + n.direction;
                    }
                    alignment = (alignment / (neighbours.len() as i32)).normalize();

                    // Cohesion
                    for n in neighbours.iter() {
                        cohesion = cohesion + wrapVector(n.location.into());
                    }

                    let mouse_pos = _window.mouse().pos();
                    cohesion = cohesion + mouse_pos;

                    cohesion = (cohesion / (neighbours.len() as i32 + 1)) - space_ship.location.into();

                    // Separation
                    for n in neighbours.iter() {
                        separation = separation + wrapVector(n.location.into()) - space_ship.location.into()
                    }

                    separation = (separation / (neighbours.len() as i32)) * -1;
                }
            }

            let len = direction * 1 + alignment * 1 + cohesion * 1 + separation * 4;
            //print!("{:?}", len);
            if len.len() > 0.0 {
                self.world.spaceships[i].direction = len / len.len();
            } else {
                self.world.spaceships[i].direction = Vector { x: 0.0, y: 0.0};
            }

        }

        fn wrap(x: f32, s: f32) -> f32 {
            ((x % s) + s) % s
        }
        // Separate
        for ref mut spaceship in &mut self.world.spaceships {

            let d = 1.0;//_window.update_rate() / 10.0;

            spaceship.location = spaceship.location  + (spaceship.direction * (d as f32));

            spaceship.location.x = wrap(spaceship.location.x, 800.0);
            spaceship.location.y = wrap(spaceship.location.y, 600.0);
        }

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        /*window.draw(&Rectangle::new((100, 100), (32, 32)), Col(Color::BLUE));
        window.draw_ex(&Rectangle::new((400, 300), (32, 32)), Col(Color::BLUE), Transform::rotate(self.rotation), 10);
        window.draw(&Circle::new((400, 300), 100), Col(Color::GREEN));
        window.draw_ex(
            &Line::new((50, 80),(600, 450)).with_thickness(2.0),
            Col(Color::RED),
            Transform::IDENTITY,
            5
        );
        window.draw_ex(
            &Triangle::new((500, 50), (450, 100), (650, 150)),
            Col(Color::RED),
            Transform::rotate(45) * Transform::scale((0.5, 0.5)),
            0
        );
        self.rotation+=1;*/

        Render::render_spaceships(&self.world, window);

        Ok(())
    }
}

fn main() {
    run::<GameState>("Draw Geometry", Vector::new(800, 600), Settings::default());
}