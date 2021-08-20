//! This module will hold all the main functions that should be called. The specific order is
//! recommended but some functions depend on the build up or tear down of others.
//!
//!
//!


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
