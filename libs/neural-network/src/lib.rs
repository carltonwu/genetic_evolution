mod layer;
mod layer_topology;
mod neuron;

use self::layer::*;
pub use self::layer_topology::*;
use self::neuron::*;
use rand::{Rng, RngCore};
use std::iter::once;

#[derive(Debug, Clone)]
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

    pub fn from_weights(
        layers: &[LayerTopology],
        weights: impl IntoIterator<Item = f32>,
    ) -> Self {
        assert!(layers.len() > 1);

        let mut weights = weights.into_iter();

        let layers = layers
            .windows(2)
            .map(|layer| {
                Layer::from_weights(
                    layer[0].neurons,
                    layer[1].neurons,
                    &mut weights,
                )
            })
            .collect();

        if weights.next().is_some() {
            panic!("got too many weights");
        }

        Self { layers }
    }

    pub fn weights(&self) -> impl Iterator<Item = f32> + '_ {
        self.layers
            .iter()
            .flat_map(|layer| layer.neurons.iter())
            .flat_map(|neuron| once(&neuron.bias).chain(&neuron.weights))
            .copied()
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

    #[test]
    fn from_weights() {
        let layers = &[
            LayerTopology { neurons: 3 },
            LayerTopology { neurons: 2 },
        ];

        let weights = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
        let network = Network::from_weights(layers, weights.clone());
        let actual: Vec<_> = network.weights().collect();

        assert_relative_eq!(actual.as_slice(), weights.as_slice());
    }

    #[test]
    fn weights() {
        let network = Network {
            layers: vec![
                Layer {
                    neurons: vec![Neuron {
                        bias: 0.1,
                        weights: vec![0.2, 0.3, 0.4],
                    }],
                },
                Layer {
                    neurons: vec![Neuron {
                        bias: 0.5,
                        weights: vec![0.6, 0.7, 0.8],
                    }],
                },
            ],
        };

        let actual: Vec<_> = network.weights().collect();
        let expected = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];

        assert_eq!(actual.as_slice(), expected.as_slice());
    }

}