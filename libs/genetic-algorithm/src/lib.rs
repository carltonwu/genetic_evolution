mod crossover;
mod chromosome;
mod individual;
mod mutation;
mod selection;

pub use self::crossover::*;
pub use self::chromosome::*;
pub use self::individual::*;
pub use self::mutation::*;
pub use self::selection::*;
use rand::{seq::SliceRandom, Rng, RngCore};

pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

impl<S> GeneticAlgorithm<S> where S: SelectionMethod, {
    pub fn new(
        selection_method: S,
        crossover_method: impl CrossoverMethod + 'static,
        mutation_method: impl MutationMethod + 'static) -> Self {
        Self {  selection_method,
                crossover_method: Box::new(crossover_method), 
                mutation_method: Box::new(mutation_method) 
             }
    }

    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> (Vec<I>, Statistics) where I: Individual {
        assert!{!population.is_empty()};

        let new_population: Vec<I>  = (0..population.len())
            .map(|_| {
                let parent_a = self.selection_method.select(rng, population).chromosome();
                let parent_b = self.selection_method.select(rng, population).chromosome();
                
                let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);

                self.mutation_method.mutate(rng, &mut child);

                I::create(child)
            })
            .collect();

        let stats = Statistics::new(population);

        (new_population, stats)
    }
}

#[derive(Clone, Debug)]
pub struct Statistics {
    pub min_fitness: f32,
    pub max_fitness: f32,
    pub avg_fitness: f32,
}

impl Statistics {
    fn new<I>(population: &[I]) -> Self where I: Individual {
        assert!(!population.is_empty());

        let mut min_fitness = population[0].fitness();
        let mut max_fitness = min_fitness;
        let mut sum_fitness = 0.0;

        for individual in population {
            let fitness = individual.fitness();

            min_fitness = min_fitness.min(fitness);
            max_fitness = max_fitness.max(fitness);
            sum_fitness = sum_fitness + fitness;
        }

        let avg_fitness = sum_fitness / population.len() as f32;

        Self { min_fitness, max_fitness, avg_fitness }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    fn individual(genes: &[f32]) -> TestIndividual {
        TestIndividual::create(genes.iter().cloned().collect())
    }
    
    #[test]
    fn genetic_algorithm() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let ga = GeneticAlgorithm::new(
            RouletteWheelSelection,
            UniformCrossover,
            GaussianMutation::new(0.5, 0.5)
        );

        let mut population = vec![
            individual(&[0.0, 0.0, 0.0]),
            individual(&[1.0, 1.0, 1.0]),
            individual(&[1.0, 2.0, 1.0]),
            individual(&[1.0, 2.0, 4.0]),
        ];

        for _ in 0..10 {
            (population, _) = ga.evolve(&mut rng, &population);
        }

        let expected_population = vec![
            individual(&[0.6002736, 1.5194247, 4.3595104]),
            individual(&[1.0955309, 2.4240465, 3.6959934]),
            individual(&[1.2753081, 2.4675508, 3.8890047]),
            individual(&[1.0225878, 2.4240465, 4.3595104]),
        ];

        assert_eq!(population, expected_population);
    }

}