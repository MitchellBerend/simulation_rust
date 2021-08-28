//! This module will hold all the main functions that should be called. The specific order is
//! recommended but some functions depend on the build up or tear down of others.
//!
//! The following order order of operations is recommended:
//!/* 
//!     1. generate_env<Environment, Agent>(pop_size)
//!     2. tick() (or tick_collect)
//!     3. collect()
//!*/
//! The final collect call will write collect all the agent and environment data and write it to a
//! file with the naming convention <pid>.json.


use std::thread;

use color_eyre::Report;

use crate::traits::{Environment, Agent};


pub fn generate_env<E: 'static + Environment, A: 'static + Agent>(pop_size: u64) -> Result<Box<dyn Environment>, Report> {
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

pub fn generate_tick_collect<E: 'static + Environment, A: 'static + Agent>(pop_size: u64, ticks: u64) -> Result<(), Report> {
    let mut v = vec!();
    for _ in 0..100 {
        v.push(thread::spawn(move || -> Result<(), Report> {
            let mut env = generate_env::<E, A>(pop_size)?;
            for _ in 0..ticks {
                env = tick(env)?;
            }
            collect(env)?;
            Ok(())
        }));
    }
    for handle in v {
        handle.join().unwrap().unwrap();
    }
    Ok(())
}
