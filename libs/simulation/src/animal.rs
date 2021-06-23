use nalgebra::{Point2, Rotation2};
use lib_neural_network as nn;
use nn::LayerTopology;
use rand::{Rng, RngCore};

use crate::Eye;

#[derive(Debug)]
pub struct Animal {
    crate position: Point2<f32>,
    crate rotation: Rotation2<f32>,
    crate speed: f32,
    crate eye: Eye,
    crate brain: nn::Network,
}

impl Animal {
    crate fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = nn::Network::random(&[LayerTopology {
            neurons: eye.cells()
        },
        LayerTopology {
            neurons: eye.cells() * 2
        },
        LayerTopology {
            neurons: 2
        }]);

        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
            eye,
            brain,
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
}
