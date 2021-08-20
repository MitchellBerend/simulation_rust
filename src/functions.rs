//! This module will hold all the main functions that should be called. The specific order is
//! recommended but some functions depend on the build up or tear down of others.
//!
//!
//!


use color_eyre::Report;

use crate::traits::{Environment, Agent};


pub fn generate_env<E: 'static + Environment, A: 'static + Agent>(population_size: u16) -> Result<Box<dyn Environment>, Report> {
    let mut pop: Vec<Box<dyn Agent>> = vec!();
    for _ in 0..population_size {
        let agent: Box<dyn Agent> = A::generate()?;
        pop.push(agent);
    }
    let env: Box<E> = E::generate(pop)?;
    Ok(env)
}


#[allow(dead_code)]
pub fn tick(mut environment: Box<dyn Environment>) -> Result<(), Report> {
    (*environment).tick()?;
    Ok(())
}
