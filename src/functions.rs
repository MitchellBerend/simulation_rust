//! This module will hold all the main functions that should be called. The specific order is
//! recommended but some functions depend on the build up or tear down of others.
//!
//!
//!


use std::process;
use std::fs::File;
use std::io::prelude::*;

use color_eyre::Report;

use crate::traits::{Environment, Agent};


fn create_agent<T: 'static + Agent>() -> Result<Box<dyn Agent>, Report> {
    Ok(T::generate()?)
}


pub fn generate_env<E: 'static + Environment, A: 'static + Agent>() -> Result<Box<E>, Report> {
    let env = E::generate::<A>()?;
    Ok(Box::new(env))
}


pub fn collect_data<T: Environment>(environment: T) -> Result<(), Report> {
    /* let pid = process::id();
    for agent in environment.population() {
        let mut file = File::create(pid.to_string())?;
        file.write_all(b"test")?;
    }
    */
    Ok(())
}


