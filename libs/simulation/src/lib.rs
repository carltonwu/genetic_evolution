use nalgebra as na;

use rand::{Rng, RngCore};

pub struct Simulation {
    world: World,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world : World::random(rng),
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_collisions(rng);
        self.process_movement();
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for agent in &mut self.world.agents {
            for food in &mut self.world.foods {
                let distance = na::distance(&agent.position, &food.position);
            
                if distance <= 0.015 {
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
}

#[derive(Debug)]
pub struct World {
    agents: Vec<Agent>,
    foods: Vec<Food>,
}

impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let agents = (0..40)
            .map(|_| Agent::random(rng))
            .collect();

        let foods = (0..60)
            .map(|_| Food::random(rng))
            .collect();

        Self { agents, foods}
    }

    pub fn agents(&self) -> &[Agent] {
        &self.agents
    }

    pub fn foods(&self) -> &[Food] {
        &self.foods
    }
}

#[derive(Debug, Clone)]
pub struct Agent {
    position: na::Point2<f32>,
    rotation: na::Rotation2<f32>,
    speed: f32,
}

impl Agent {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.01,
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }
}

#[derive(Debug, Clone)]
pub struct Food {
    position: na::Point2<f32>,
}

impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
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
        simulation.step();
        simulation.step();
        simulation.step();
        simulation.step();
        let agent_after = simulation.world.agents().first().unwrap().clone();

        assert_ne!(agent_before.position.x, agent_after.position.x);
    }
}