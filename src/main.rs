mod tools;
mod verilog;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Sigma Verilog MCP Server starting...");
    Ok(())
}