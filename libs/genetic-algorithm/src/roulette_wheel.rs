use rand::{prelude::SliceRandom, RngCore};

use crate::Individual;
use crate::SelectionMethod;

pub struct RouletteWheelSelection;

impl RouletteWheelSelection {
    pub fn new() -> Self {
        Self
    }
}

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("got empty population, not going to work")
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, iter::FromIterator};

    use crate::{
        chromosome::Chromosome, gaussian_mutation::GaussianMutation,
        uniform_crossover::UniformCrossover, GeneticAlgorithm,
    };

    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[derive(Clone, Debug, PartialEq)]
    pub enum TestIndividual {
        WithFitness { fitness: f32 },
        WithChromosome { chromosome: Chromosome },
    }

    impl TestIndividual {
        pub fn new(fitness: f32) -> Self {
            Self::WithFitness { fitness }
        }
    }

    impl Individual for TestIndividual {
        fn fitness(&self) -> f32 {
            match self {
                TestIndividual::WithFitness { fitness } => *fitness,
                TestIndividual::WithChromosome { chromosome } => chromosome.iter().sum(),
            }
        }

        fn chromosome(&self) -> &Chromosome {
            match self {
                TestIndividual::WithFitness { .. } => {
                    panic!("not supported for `WithFitness`")
                }
                TestIndividual::WithChromosome { chromosome } => chromosome,
            }
        }

        fn from_chromosome(chromosome: Chromosome) -> Self {
            Self::WithChromosome { chromosome }
        }
    }

    #[test]
    fn test_selection() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let method = RouletteWheelSelection::new();

        let population = vec![
            TestIndividual::new(2.0),
            TestIndividual::new(1.0),
            TestIndividual::new(4.0),
            TestIndividual::new(3.0),
        ];

        let actual_histogram: BTreeMap<i32, _> = (0..1_000)
            .map(|_| method.select(&mut rng, &population))
            .fold(Default::default(), |mut histogram, individual| {
                *histogram.entry(individual.fitness() as _).or_default() += 1;
                histogram
            });

        let expected_histogram = BTreeMap::from_iter(vec![
            // fitness, count
            (1, 98),
            (2, 202),
            (3, 278),
            (4, 422),
        ]);

        assert_eq!(actual_histogram, expected_histogram);
    }

    fn individual(genes: &[f32]) -> TestIndividual {
        let chromosome = genes.iter().cloned().collect();
        TestIndividual::from_chromosome(chromosome)
    }

    #[test]
    fn test_mutation() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let ga = GeneticAlgorithm::new(
            RouletteWheelSelection::new(),
            UniformCrossover::new(),
            GaussianMutation::new(0.5, 0.5),
        );

        let mut population = vec![
            individual(&[0.0, 0.0, 0.0]), // fitness: 0.0
            individual(&[1.0, 1.0, 1.0]), // fitness: 3.0
            individual(&[1.0, 2.0, 1.0]), // fitness: 4.0
            individual(&[1.0, 2.0, 4.0]), // fitness: 7.0
        ];

        for _ in 0..10 {
            population = ga.evolve(&mut rng, &population);
        }

        let expected_population = vec![
            individual(&[1.606008, 2.789879, 3.6941864]), // fitness: ~ 8.0
            individual(&[1.0839049, 2.4461222, -0.8869108]), //fitness: ~ 7.1
            individual(&[0.99193525, 2.588976, 3.5712361]), // fitness: ~ 8.0
            individual(&[1.646358, 2.392836, 3.9752667]), // fitness: ~ 7.8
        ];

        assert_eq!(population, expected_population);
    }
}
