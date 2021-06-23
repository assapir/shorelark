use rand::{Rng, RngCore};

use crate::{chromosome::Chromosome, MutationMethod};

#[derive(Clone, Debug)]
pub struct GaussianMutation {
    /// probability of mutating:
    /// - 0.0 => nothing will be mutated
    /// - 1.0 => everything will be mutated
    chance: f32,

    /// Magnitude of mutation:
    /// - 0.0 => touched genes will not be mutated
    /// - 3.0 => touched genes will be mutated += or -= by at most 3.0
    coefficient: f32,
}

impl GaussianMutation {
    pub fn new(chance: f32, coefficient: f32) -> Self {
        assert!(chance >= 0.0 && chance <= 1.0);

        Self {
            chance,
            coefficient,
        }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
        child.iter_mut().for_each(|gene| {
            let sign = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };

            if rng.gen_bool(self.chance as _) {
                *gene += sign * self.coefficient * rng.gen::<f32>();
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    fn actual(chance: f32, coefficient: f32) -> Vec<f32> {
        let mut child = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_iter().collect();

        let mut rng = ChaCha8Rng::from_seed(Default::default());

        GaussianMutation::new(chance, coefficient).mutate(&mut rng, &mut child);

        child.into_iter().collect()
    }

    mod given_zero_chance {

        mod and_zero_coefficient {
            use crate::gaussian_mutation::tests::actual;

            #[test]
            fn does_not_change_genes() {
                let actual = actual(0.0, 0.0);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }

        mod and_non_zero_coefficient {
            use crate::gaussian_mutation::tests::actual;

            #[test]
            fn does_not_change_genes() {
                let actual = actual(0.0, 0.5);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
    }

    mod given_50_50_chance {
        mod and_zero_coefficient {
            use crate::gaussian_mutation::tests::actual;

            #[test]
            fn does_not_change_genes() {
                let actual = actual(0.5, 0.0);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }

        mod and_non_zero_coefficient {
            use crate::gaussian_mutation::tests::actual;

            #[test]
            fn slightly_changes_not_change_genes() {
                let actual = actual(0.5, 0.5);
                let expected = vec![1.0, 2.2243752, 3.0, 3.8403194, 5.0];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
    }

    mod given_max_chance {
        mod and_zero_coefficient {
            use crate::gaussian_mutation::tests::actual;
            #[test]
            fn does_not_change_genes() {
                let actual = actual(1.0, 0.0);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }

        mod and_non_zero_coefficient {
            use crate::gaussian_mutation::tests::actual;
            #[test]
            fn entirely_changes_genes() {
                let actual = actual(1.0, 3.0);
                let expected = vec![-1.7271891, 1.3027526, 4.3462505, 4.296925, 7.1678534];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
    }
}
