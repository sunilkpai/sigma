use anyhow::Result;

pub struct VerilogParser;

impl VerilogParser {
    pub fn new() -> Self {
        Self
    }
    
    pub fn parse_file(&self, _path: &str) -> Result<Vec<String>> {
        Ok(vec![])
    }
}