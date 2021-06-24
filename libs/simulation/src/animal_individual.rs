use ga::{chromosome::Chromosome, Individual};
use lib_genetic_algorithm as ga;
use rand::RngCore;

use crate::Animal;

pub struct AnimalIndividual {
    fitness: f32,
    chromosome: Chromosome,
}

impl AnimalIndividual {
    pub fn from_animal(animal: &Animal) -> Self {
        Self {
            fitness: animal.ate as f32,
            chromosome: animal.as_chromosome(),
        }
    }

    pub fn into_animal(self, rng: &mut dyn RngCore) -> Animal {
        Animal::from_chromosome(self.chromosome, rng)
    }
}

impl Individual for AnimalIndividual {
    fn fitness(&self) -> f32 {
        self.fitness
    }

    fn chromosome(&self) -> &Chromosome {
        &self.chromosome
    }

    fn from_chromosome(chromosome: Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome,
        }
    }
}
