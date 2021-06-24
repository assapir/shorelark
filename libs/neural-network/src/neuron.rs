use rand::Rng;

#[derive(Debug)]
pub struct Neuron {
    crate bias: f32,
    crate weights: Vec<f32>,
}

impl Neuron {
    pub fn new(bias: f32, weights: Vec<f32>) -> Self {
        Self { bias, weights }
    }

    crate fn random(rng: &mut dyn rand::RngCore, output_size: usize) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..output_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }

    crate fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(
            inputs.len(),
            self.weights.len(),
            "got {} inputs, but {} inputs were expected",
            inputs.len(),
            self.weights.len()
        );

        let output: f32 = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum();

        (output + self.bias).max(0.0)
    }

    pub fn from_weights(output_neurons: usize, weights: &mut dyn Iterator<Item = f32>) -> Self {
        let bias = weights.next().expect("got not enough weights");

        let weights = (0..output_neurons)
            .map(|_| weights.next().expect("got not enough weights"))
            .collect();

        Self { bias, weights }
    }
}
