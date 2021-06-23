#![feature(crate_visibility_modifier)]

use rand::RngCore;

use crate::layer::Layer;

mod layer;
mod neuron;

#[derive(Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn from_weights(
        layers: &[LayerTopology],
        weights: impl IntoIterator<Item = f32>,
    ) -> Network {
        assert!(layers.len() > 1);

        let mut weights = weights.into_iter();

        let layers = layers
            .windows(2)
            .map(|layers| Layer::from_weights(layers[0].neurons, layers[1].neurons, &mut weights))
            .collect();

        if weights.next().is_some() {
            panic!("got too many wrights");
        }

        Self { layers }
    }

    pub fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        assert!(
            layers.len() > 1,
            "Networks with one layer doesn't make any sense"
        );

        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();

        Self { layers }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    pub fn weights(&self) -> impl Iterator<Item = f32> + '_ {
        use std::iter::once;

        self.layers
            .iter()
            .flat_map(|layer| layer.neurons.iter())
            .flat_map(|neuron| once(&neuron.bias).chain(&neuron.weights))
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod random {
        use crate::neuron::Neuron;

        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        #[test]
        fn test() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let neuron = Neuron::random(&mut rng, 4);

            approx::assert_relative_eq!(neuron.bias, -0.6255188);
            approx::assert_relative_eq!(
                neuron.weights.as_slice(),
                [0.67383957, 0.8181262, 0.26284897, 0.5238807].as_ref()
            );
        }
    }

    mod propagate {
        use crate::neuron::Neuron;

        #[test]
        fn test() {
            let neuron = Neuron {
                bias: 0.5,
                weights: vec![-0.3, 0.8],
            };

            approx::assert_relative_eq!(neuron.propagate(&[-10.0, -10.0]), 0.0);

            approx::assert_relative_eq!(
                neuron.propagate(&[0.5, 1.0]),
                (-0.3 * 0.5) + (0.8 * 1.0) + 0.5
            );
        }
    }

    mod weights {
        use crate::neuron::Neuron;

        use super::*;

        #[test]
        fn test() {
            let network = Network::new(vec![
                Layer::new(vec![Neuron::new(0.1, vec![0.2, 0.3, 0.4])]),
                Layer::new(vec![Neuron::new(0.5, vec![0.6, 0.7, 0.8])]),
            ]);

            let actual: Vec<_> = network.weights().collect();
            let expected = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];

            approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
        }
    }

    mod from_weights {
        use super::*;

        #[test]
        fn test() {
            let layers = &[LayerTopology { neurons: 3 }, LayerTopology { neurons: 2 }];

            let weights = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
            let network = Network::from_weights(layers, weights.clone());
            let actual: Vec<_> = network.weights().collect();

            approx::assert_relative_eq!(actual.as_slice(), weights.as_slice());
        }
    }
}
