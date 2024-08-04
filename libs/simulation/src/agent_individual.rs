use crate::*;

pub struct AgentIndividual {
    fitness: f32,
    chromosome: ga::Chromosome,
}

impl AgentIndividual {
    pub fn from_agent(agent: &Agent) -> Self {
        Self {
            fitness: agent.satiation as f32,
            chromosome: agent.as_chromosome(),
        }
    }

    pub fn into_agent(self, rng: &mut dyn RngCore) -> Agent {
        Agent::from_chromosome(self.chromosome, rng)
    }
}

impl ga::Individual for AgentIndividual {
    fn create(chromosome: ga::Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome,
        }
    }

    fn chromosome(&self) -> &ga::Chromosome {
        &self.chromosome
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }
}