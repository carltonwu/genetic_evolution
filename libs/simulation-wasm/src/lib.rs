use wasm_bindgen::prelude::*;
use lib_simulation as sim;
use rand::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(&mut rng);

        Self { rng, sim }
    }

    pub fn world(&self) -> World {
        World::from(self.sim.world())
    }

    pub fn step(&mut self) {
        self.sim.step(&mut self.rng);
    }

    pub fn train(&mut self) -> String {
        let stats = self.sim.train(&mut self.rng);

        format!(
            "min={:.2}, max={:.2}, avg={:.2}",
            stats.min_fitness,
            stats.max_fitness,
            stats.avg_fitness,
        )
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct World {
    #[wasm_bindgen(getter_with_clone)]
    pub agents: Vec<Agent>,
    #[wasm_bindgen(getter_with_clone)]
    pub foods: Vec<Food>,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Agent {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Food {
    pub x: f32,
    pub y: f32,
}

impl From<&sim::World> for World {
    fn from(world: &sim::World) -> Self {
        let agents = world.agents().iter().map(Agent::from).collect();
        let foods = world.foods().iter().map(Food::from).collect();

        Self { agents, foods }
    }
}

impl From<&sim::Agent> for Agent {
    fn from(agent: &sim::Agent) -> Self {
        Self { x: agent.position().x, y: agent.position().y, rotation: agent.rotation().angle() }
    }
}

impl From<&sim::Food> for Food {
    fn from(food: &sim::Food) -> Self {
        Self { x: food.position().x, y: food.position().y }
    }
}