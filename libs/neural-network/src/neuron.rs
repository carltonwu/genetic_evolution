use crate::*;

#[derive(Clone, Debug)]
pub struct Neuron {
    pub(crate) bias: f32,
    pub(crate) weights: Vec<f32>,
}

impl Neuron {
    pub fn random(rng: &mut dyn RngCore, input_size: usize) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..input_size)
                                        .into_iter()
                                        .map(|_| rng.gen_range(-1.0..=1.0))
                                        .collect();

        return Self{ bias, weights };
    }

    pub fn from_weights(
        input_size: usize,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        let bias = weights.next().expect("got not enough weights");

        let weights = (0..input_size)
            .map(|_| weights.next().expect("got not enough weights"))
            .collect();

        Self { bias, weights }
    }

    pub fn propogate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());

       let mut output = inputs
                        .iter()
                        .zip(&self.weights)
                        .map(|(input, weight)| input * weight)
                        .sum::<f32>();

        output += self.bias;

        return output.max(0.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use approx::assert_relative_eq;

    #[test]
    fn neuron_random() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let neuron = Neuron::random(&mut rng, 4);

        assert_relative_eq!(neuron.bias, -0.6255188);
        assert_relative_eq!(neuron.weights.as_slice(),
            [0.67383957, 0.8181262, 0.26284897, 0.5238807].as_ref());
    }

    #[test]
    fn neuron_propogate() {
        let neuron = Neuron {
            bias: 0.5,
            weights: vec![-0.3, 0.8],
        };

        assert_relative_eq!(neuron.propogate(&[-10.0, -10.0]),
            0.0,
        );

        assert_relative_eq!(neuron.propogate(&[0.5, 1.0]),
            (-0.3 * 0.5) + (0.8 * 1.0) + 0.5,
        );
    }
}