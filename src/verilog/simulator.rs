use anyhow::Result;

pub struct SimulationResult {
    pub success: bool,
    pub output: String,
}

pub struct Simulator;

impl Simulator {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn run(&self, _executable: &str) -> Result<SimulationResult> {
        Ok(SimulationResult {
            success: true,
            output: "simulation output".to_string(),
        })
    }
}