#![feature(crate_visibility_modifier)]

mod animal;
mod eye;
mod food;
mod world;

use std::f32::consts::FRAC_PI_2;

pub use animal::Animal;
pub use eye::Eye;
pub use food::Food;
pub use world::World;
use na::{Rotation2, Vector2};
use nalgebra as na;
use rand::Rng;
use rand::RngCore;

const SPEED_MIN: f32 = 0.001;
const SPEED_MAX: f32 = 0.005;
const SPEED_ACCEL: f32 = 0.2;
const ROTATION_ACCEL: f32 = FRAC_PI_2;

pub struct Simulation {
    world: World,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng),
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_collisions(rng);
        self.process_brain();
        self.process_movement();
    }

    fn process_movement(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation() * Vector2::new(animal.speed(), 0.0);

            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = na::distance(&animal.position(), &food.position());

                if distance <= 0.01 {
                    food.position = rng.gen();
                }
            }
        }
    }

    fn process_brain(&mut self) {
        for animal in &mut self.world.animals {
            let vision = animal.eye.process_vision(animal.position, animal.rotation, &self.world.foods);
            let response = animal.brain.propagate(vision);
            let speed = response[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);
            let rotation = response[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);

            animal.speed = (animal.speed() + speed).clamp(SPEED_MIN, SPEED_MAX);
            animal.rotation = Rotation2::new(animal.rotation.angle() + rotation);

        }
    }
}
