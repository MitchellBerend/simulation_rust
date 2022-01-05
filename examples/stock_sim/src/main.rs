use sim_rust::functions as sim;
use sim_rust::{Environment, Agent};

mod agent;
use agent::Stock;

fn main() -> Result<(), &'static str> {
    let mut env: Box<dyn Environment> = sim::generate_default_env::<Stock>(0)?;
    let agents: Vec<Box<dyn Agent>> = vec![
        Stock::new("examples/stock_sim/config/aapl.toml")?,
        Stock::new("examples/stock_sim/config/amd.toml")?,
        Stock::new("examples/stock_sim/config/amzn.toml")?,
    ];
    env.add_agents(agents)?;
    for _ in 0..260 {
        sim::tick(&mut env)?;
    }
    println!("[");
    sim::collect(env)?;
    println!("]");
    Ok(())
}
