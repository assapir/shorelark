use lib_genetic_algorithm::chromosome::Chromosome;
use nalgebra::{Point2, Rotation2};
use rand::{Rng, RngCore};

use crate::{Brain, Eye};

#[derive(Debug)]
pub struct Animal {
    crate position: Point2<f32>,
    crate rotation: Rotation2<f32>,
    crate speed: f32,
    crate eye: Eye,
    crate brain: Brain,
    crate ate: usize,
}

impl Animal {
    pub fn new(eye: Eye, brain: Brain, rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.02,
            eye,
            brain,
            ate: 0,
        }
    }

    crate fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::random(rng, &eye);

        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
            eye,
            brain,
            ate: 0,
        }
    }

    pub fn position(&self) -> Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> Rotation2<f32> {
        self.rotation
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }

    pub fn as_chromosome(&self) -> Chromosome {
        self.brain.as_chromosome()
    }

    crate fn from_chromosome(
        chromosome: Chromosome,
        rng: &mut dyn RngCore,
    ) -> Self {
        let eye = Eye::default();
        let brain = Brain::from_chromosome(chromosome, &eye);

        Self::new(eye, brain, rng)
    }
}
