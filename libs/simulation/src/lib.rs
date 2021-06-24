#![feature(crate_visibility_modifier)]

mod animal;
mod animal_individual;
mod brain;
mod eye;
mod food;
mod world;

use std::f32::consts::FRAC_PI_2;

pub use animal::Animal;
use animal_individual::AnimalIndividual;
pub use brain::Brain;
pub use eye::Eye;
pub use food::Food;
use ga::{GeneticAlgorithm, gaussian_mutation::GaussianMutation, roulette_wheel::RouletteWheelSelection, statistics::Statistics, uniform_crossover::UniformCrossover};
use lib_genetic_algorithm as ga;
use na::{Rotation2, Vector2};
use nalgebra as na;
use rand::Rng;
use rand::RngCore;
pub use world::World;

const SPEED_MIN: f32 = 0.001;
const SPEED_MAX: f32 = 0.005;
const SPEED_ACCEL: f32 = 0.2;
const ROTATION_ACCEL: f32 = FRAC_PI_2;
const GENERATION_LENGTH: usize = 2500;

pub struct Simulation {
    world: World,
    ga: GeneticAlgorithm<RouletteWheelSelection>,
    age: usize,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let world = World::random(rng);

        let ga = GeneticAlgorithm::new(
            RouletteWheelSelection::new(),
            UniformCrossover::new(),
            GaussianMutation::new(0.01, 0.3),
        );
        Self { world, ga, age: 0 }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) -> Option<Statistics>{
        self.process_collisions(rng);
        self.process_brain();
        self.process_movement();

        self.age += 1;
        if self.age >= GENERATION_LENGTH {
            Some(self.evolve(rng))
        } else {
            None
        }
    }

    pub fn train(&mut self, rng: &mut dyn RngCore) -> Statistics{
        loop {
            if let Some(stat) = self.step(rng) {
                return stat;
            }
        }
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
                    animal.ate += 1;
                    food.position = rng.gen();
                }
            }
        }
    }

    fn process_brain(&mut self) {
        for animal in &mut self.world.animals {
            let vision =
                animal
                    .eye
                    .process_vision(animal.position, animal.rotation, &self.world.foods);
            let response = animal.brain.nn.propagate(vision);
            let speed = response[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);
            let rotation = response[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);

            animal.speed = (animal.speed() + speed).clamp(SPEED_MIN, SPEED_MAX);
            animal.rotation = Rotation2::new(animal.rotation.angle() + rotation);
        }
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) -> Statistics {
        self.age = 0;
        let current_population: Vec<AnimalIndividual> = self
            .world
            .animals
            .iter()
            .map(AnimalIndividual::from_animal)
            .collect();

        let (evolved_population, stats)= self.ga.evolve(rng, &current_population);
        self.world.animals = evolved_population
            .into_iter()
            .map(|individual| individual.into_animal(rng))
            .collect();

        for food in &mut self.world.foods {
            food.position = rng.gen()
        };

        stats
    }
}
