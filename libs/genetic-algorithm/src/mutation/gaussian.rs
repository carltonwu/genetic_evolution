use crate::*;

#[derive(Clone, Debug)]
pub struct GaussianMutation {
    mutation_probability: f32,
    mutation_coefficient: f32,
}

impl GaussianMutation {
    pub fn new(mutation_probability: f32, mutation_coefficient: f32) -> Self {
        assert!(mutation_probability >= 0.0 && mutation_coefficient <= 1.0);

        Self { mutation_probability, mutation_coefficient }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
        for gene in child.iter_mut() {
            if rng.gen_bool(self.mutation_probability as f64) {
                *gene += rng.gen_range(-1.0..=1.0) * self.mutation_coefficient;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    fn actual(chance: f32, coeff: f32) -> Vec<f32> {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut child = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_iter().collect();

        GaussianMutation::new(chance, coeff).mutate(&mut rng, &mut child);

        child.into_iter().collect()
    }

    mod given_zero_chance {
        use approx::assert_relative_eq;

        fn actual(coeff: f32) -> Vec<f32> {
            super::actual(0.0, coeff)
        }
        mod and_zero_coefficient {
            use super::*;

            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = actual(0.0);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }

        mod and_nonzero_coefficient {
            use super::*;

            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = actual(0.5);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
    }

    mod given_fifty_fifty_chance {
        use approx::assert_relative_eq;

        fn actual(coeff: f32) -> Vec<f32> {
            super::actual(0.5, coeff)
        }

        mod and_zero_coefficient {
            use super::*;

            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = actual(0.0);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }

        mod and_nonzero_coefficient {
            use super::*;

            #[test]
            fn slightly_changes_the_original_chromosome() {
                let actual = actual(0.1);
                let expected = vec![1.0, 2.0, 3.006937, 3.9511204, 5.027546];

                assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
    }

    mod given_max_chance {
        use approx::assert_relative_eq;

        fn actual(coeff: f32) -> Vec<f32> {
            super::actual(1.0, coeff)
        }

        mod and_zero_coefficient {
            use super::*;

            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = actual(0.0);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }

        mod and_nonzero_coefficient {
            use super::*;

            #[test]
            fn entirely_changes_the_original_chromosome() {
                let actual = actual(0.1);
                let expected = vec![0.93744814, 2.067384, 3.0818126, 4.0262847, 5.052388];

                assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
    }
}