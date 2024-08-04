use crate::*;

#[derive(Debug)]
pub struct World {
    pub(crate) agents: Vec<Agent>,
    pub(crate) foods: Vec<Food>,
}

impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let agents = (0..10)
            .map(|_| Agent::random(rng))
            .collect();

        let foods = (0..30)
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