//! This module will hold all the main functions that should be called. The specific order is
//! recommended but some functions depend on the build up or tear down of others.
//!
//! The following order order of operations is recommended:
//! 
//!     1. generate_env<Environment, Agent>(pop_size)
//!     2. tick() (or tick_collect)
//!     3. collect()
//!
//! The final collect call will write collect all the agent and environment data and write it to a
//! file with the naming convention <pid>.json.



use color_eyre::Report;

use crate::traits::{Environment, Agent};


pub fn generate_env<E: 'static + Environment, A: 'static + Agent>(pop_size: u16) -> Result<Box<dyn Environment>, Report> {
    let mut pop: Vec<Box<dyn Agent>> = vec!();
    for _ in 0..pop_size {
        let agent: Box<dyn Agent> = A::generate()?;
        pop.push(agent);
    }
    let env: Box<E> = E::generate(pop)?;
    Ok(env)
}


pub fn tick(mut environment: Box<dyn Environment>) -> Result<Box<dyn Environment>, Report> {
    (*environment).tick()?;
    Ok(environment)
}

pub fn tick_collect(mut environment: Box<dyn Environment>) -> Result<Box<dyn Environment>, Report> {
    (*environment).tick()?;
    (*environment).collect()?;
    Ok(environment)
}


pub fn collect(environment: Box<dyn Environment>) -> Result<Box<dyn Environment>, Report> {
    (*environment).collect()?;
    Ok(environment)
}

