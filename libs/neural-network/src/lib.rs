mod layer;
mod layer_topology;
mod neuron;

use self::layer::*;
pub use self::layer_topology::*;
use self::neuron::*;
use rand::{Rng, RngCore};

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
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




#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use approx::assert_relative_eq;

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