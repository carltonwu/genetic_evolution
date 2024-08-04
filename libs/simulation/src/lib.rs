mod agent;
mod agent_individual;
mod eye;
mod brain;
mod food;
mod world;

pub use self::agent::*;
pub use self::agent_individual::*;
pub use self::eye::*;
pub use self::brain::*;
pub use self::food::*;
pub use self::world::*;
pub use self::nn::*;

use lib_genetic_algorithm as ga;
use lib_neural_network as nn;
use nalgebra as na;
use rand::{Rng, RngCore};
use std::f32::consts::FRAC_PI_4;

const SPEED_MIN: f32 = 0.001;
const SPEED_MAX: f32 = 0.005;
const SPEED_ACCEL: f32 = 0.2;
const ROTATION_ACCEL: f32 = FRAC_PI_4;

const GENERATION_LIMIT: usize = 2500;

pub struct Simulation {
    world: World,
    ga: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    age: usize,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let world = World::random(rng);
        let ga = ga::GeneticAlgorithm::new(
            ga::RouletteWheelSelection,
            ga::UniformCrossover,
            ga::GaussianMutation::new(0.01, 0.3),
        );

        Self {
            world,
            ga,
            age: 0,
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) -> Option<ga::Statistics> {
        self.process_collisions(rng);
        self.process_brains();
        self.process_movement();

        self.age = self.age + 1;

        if self.age > GENERATION_LIMIT {
            Some(self.evolve(rng))
        } else {
            None
        }
    }

    pub fn train(&mut self, rng: &mut dyn RngCore) -> ga::Statistics {
        loop {
            if let Some(summary) = self.step(rng) {
                return summary;
            }
        }
    }

    fn process_brains(&mut self) {
        for agent in &mut self.world.agents {
            let vision = agent.eye.process_vision(
                agent.position,
                agent.rotation,
                &self.world.foods,
            );

            let response = agent.brain.nn.propogate(vision);

            let speed = response[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);
            let rotation = response[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);

            agent.speed = (agent.speed + speed).clamp(SPEED_MIN, SPEED_MAX);
            agent.rotation = na::Rotation2::new(agent.rotation.angle() + rotation);

        }
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for agent in &mut self.world.agents {
            for food in &mut self.world.foods {
                let distance = na::distance(&agent.position, &food.position);
            
                if distance <= 0.02 {
                    agent.satiation += 1;
                    food.position = rng.gen()
                }
            }
        }
    }

    fn process_movement(&mut self) {
        for agent in &mut self.world.agents {
            agent.position += agent.rotation * na::Vector2::new(0.0, agent.speed);

            agent.position.x = na::wrap(agent.position.x, 0.0, 1.0);
            agent.position.y = na::wrap(agent.position.y, 0.0, 1.0);

        }
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) -> ga::Statistics {
        self.age = 0;

        let current_population: Vec<_> = self
            .world
            .agents
            .iter()
            .map(AgentIndividual::from_agent)
            .collect();

        let (evolved_population, stats) = self.ga.evolve(
            rng,
            &current_population,
        );

        self.world.agents = evolved_population
            .into_iter()
            .map(|individual| individual.into_agent(rng))
            .collect();

        for food in &mut self.world.foods {
            food.position = rng.gen();
        }

        stats
    }
}


#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use super::*;
    use crate::World;


    #[test]
    fn test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut simulation = Simulation::random(&mut rng);

        let agent_before = simulation.world.agents().first().unwrap().clone();
        simulation.step(&mut rng);
        simulation.step(&mut rng);
        simulation.step(&mut rng);
        simulation.step(&mut rng);
        let agent_after = simulation.world.agents().first().unwrap().clone();

        assert_ne!(agent_before.position.x, agent_after.position.x);
    }
}