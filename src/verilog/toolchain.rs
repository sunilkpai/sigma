use anyhow::Result;

pub enum Simulator {
    Icarus,
    Verilator, 
    ModelSim,
}

pub struct Toolchain {
    pub simulator: Simulator,
}

impl Toolchain {
    pub fn detect() -> Result<Self> {
        Ok(Self {
            simulator: Simulator::Icarus,
        })
    }
    
    pub fn build(&self, _files: &[&str]) -> Result<String> {
        Ok("build output".to_string())
    }
}