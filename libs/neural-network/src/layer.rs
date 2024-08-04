use crate::*;

#[derive(Clone, Debug)]
pub struct Layer {
    pub(crate) neurons: Vec<Neuron>,
}

impl Layer {
    pub fn random(rng: &mut dyn RngCore, input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
                        .into_iter()
                        .map(|_| Neuron::random(rng, input_size))
                        .collect();

        return Self{ neurons };
    }

    pub fn from_weights(
        input_size: usize,
        output_size: usize,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::from_weights(input_size, weights))
            .collect();

        Self { neurons }
    }

    pub fn propogate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propogate(&inputs))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use approx::assert_relative_eq;

    #[test]
    fn layer_random() {
        let mut rng: ChaCha8Rng = ChaCha8Rng::from_seed(Default::default());
        let layer = Layer::random(&mut rng, 4, 2);
        
        assert_relative_eq!(layer.neurons[1].bias, -0.53516835);
        assert_relative_eq!(layer.neurons[1].weights.as_slice(),
            [0.069369674, -0.7648182, -0.102499366, -0.48879617].as_ref());
    }

    #[test]
    fn layer_propogate() {
        let mut rng: ChaCha8Rng = ChaCha8Rng::from_seed(Default::default());
        let layer = Layer::random(&mut rng, 4, 2);

        assert_relative_eq!(layer.propogate(vec![0.5, 1.0, 0.25, 1.0]).as_slice(),
            [(((0.67383957 * 0.5) + (0.8181262 * 1.0) + (0.26284897 * 0.25) + 
            (0.5238807 * 1.0) + -0.6255188) as f32).max(0.0), 
            (((0.069369674 * 0.5) + (-0.7648182 * 1.0) + (-0.102499366 * 0.25) + 
            (-0.48879617 * 1.0) + -0.53516835) as f32).max(0.0)].as_ref());
    }

}