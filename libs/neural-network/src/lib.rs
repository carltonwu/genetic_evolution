use rand::{Rng, RngCore};

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

pub struct LayerTopology {
    pub neurons: usize,
}

impl Network {
    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);

        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();

        return Self{ layers }
    }

    pub fn propogate(&self, mut inputs: Vec<f32>) -> Vec<f32> {
        for layer in &self.layers {
            inputs = layer.propogate(inputs);
        }

        return inputs;
    }
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    fn random(rng: &mut dyn RngCore, input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
                        .into_iter()
                        .map(|_| Neuron::random(rng, input_size))
                        .collect();

        return Self{ neurons };
    }

    fn propogate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propogate(&inputs))
            .collect()
    }
}

#[derive(Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Neuron {
    fn random(rng: &mut dyn RngCore, input_size: usize) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..input_size)
                                        .into_iter()
                                        .map(|_| rng.gen_range(-1.0..=1.0))
                                        .collect();

        return Self{ bias, weights };
    }

    fn propogate(&self, inputs: &[f32]) -> f32 {
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

    #[test]
    fn network_random() {
        let mut rng: ChaCha8Rng = ChaCha8Rng::from_seed(Default::default());
        let network = Network::random(&mut rng,
             &[LayerTopology{ neurons: 4 }, LayerTopology{ neurons: 2 }, LayerTopology{ neurons: 1 }]);

        let neuron = &network.layers[1].neurons[0];

        assert_relative_eq!(neuron.bias, -0.19277132);
        assert_relative_eq!(neuron.weights.as_slice(),
            [-0.8020501, 0.2754606].as_ref());
    }

    #[test]
    fn network_propogate() {
        let mut rng: ChaCha8Rng = ChaCha8Rng::from_seed(Default::default());
        let network = Network::random(&mut rng,
             &[LayerTopology{ neurons: 4 }, LayerTopology{ neurons: 2 }, LayerTopology{ neurons: 1 }]);
        
        assert_relative_eq!(network.propogate(vec![0.5, 1.0, 0.25, 1.0]).as_slice(),
            [(((1.1191201275 * -0.8020501) + (0.0 * 0.2754606)) + -0.19277132 as f32).max(0.0)].as_ref())
    }

}