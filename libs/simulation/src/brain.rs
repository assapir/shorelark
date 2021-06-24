use lib_genetic_algorithm::chromosome::Chromosome;
use lib_neural_network::{LayerTopology, Network};
use rand::RngCore;

use crate::Eye;

#[derive(Debug)]
pub struct Brain {
    crate nn: Network,
}

impl Brain {
    pub fn random(rng: &mut dyn RngCore, eye: &Eye) -> Self {
        Self {
            nn: Network::random(rng,&Self::topology(eye)),
        }
    }

    crate fn as_chromosome(&self) -> Chromosome {
        self.nn.weights().collect()
    }

    crate fn from_chromosome(
        chromosome: Chromosome,
        eye: &Eye,
    ) -> Self {
        Self {
            nn: Network::from_weights(
                &Self::topology(eye),
                chromosome,
            ),
        }
    }


    fn topology(eye: &Eye) -> [LayerTopology; 3] {
        [
            LayerTopology {
                neurons: eye.cells(),
            },
            LayerTopology {
                neurons: 2 * eye.cells(),
            },
            LayerTopology { neurons: 2 },
        ]
    }
}
