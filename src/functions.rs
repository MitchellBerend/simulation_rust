//! This module will hold all the main functions that should be called. The specific order is
//! recommended but some functions depend on the build up or tear down of others.
//!
//! The following order order of operations is recommended:
//!
//!  # Example
//! ```
//! use sim_rust::Agent;
//! use sim_rust::functions::*;
//!
//! // method 1 and 2 have the exact same outcome
//!
//! // method 1  
//! let mut env_1 = generate_default_env::<ExampleAgent1>(10).unwrap();
//! tick(&mut env_1).unwrap(); //(or tick_collect)
//! collect(env_1);
//!
//!
//! // method 2
//! let mut env_2 = generate_default_tick_collect::<ExampleAgent1>(10, 1, 1).unwrap();
//!
//!
//! // method 3
//! let mut env_3 = generate_default_env::<ExampleAgent1>(10).unwrap();
//! // Any type that implements the Agent trait can be added to the environment.
//! let example_agent_2 = ExampleAgent2::generate().unwrap();
//! env_3.add_agent(example_agent_2).unwrap();
//! // even mulitple instances at once.
//! let example_agents: Vec<Box<dyn Agent>> = vec![
//!     ExampleAgent2::generate().unwrap(),
//!     ExampleAgent2::generate().unwrap(),
//!     ExampleAgent2::generate().unwrap(),
//! ];
//! env_3.add_agents(example_agents).unwrap();
//! tick(&mut env_3).unwrap();
//! collect(env_3);
//!
//!   
//! // This is just a very simple implementation of an agent.
//! struct ExampleAgent1 {
//!     age: u8,
//! }
//!
//! struct ExampleAgent2 {
//!     age: u8,
//! }
//!
//!
//! impl Agent for ExampleAgent1 {
//!     fn generate() -> Result<Box<Self> ,&'static str> {
//!         let agent = Box::new(Self {age: 1});
//!         Ok(agent)
//!     }
//!
//!     fn tick(&mut self) -> Result<(), &'static str> {
//!         self.age += 1;
//!         Ok(())
//!     }
//!
//!     fn collect(&self)  -> Result<(), &'static str> {
//!         Ok(())
//!     }
//! }
//!
//! // This is a direct copy of the implementation of ExampleAgent1
//! impl Agent for ExampleAgent2 {
//!     fn generate() -> Result<Box<Self> ,&'static str> {
//!         let agent = Box::new(Self {age: 1});
//!         Ok(agent)
//!     }
//!
//!     fn tick(&mut self) -> Result<(), &'static str> {
//!         self.age += 1;
//!         Ok(())
//!     }
//!
//!     fn collect(&self)  -> Result<(), &'static str> {
//!         Ok(())
//!     }
//! }
//!```

use std::thread;

use crate::traits::{Agent, Environment};

/// Generates a standard environment with a specified agent.
/// This environment is the standard implementation and does
///  not provide any custom behavior.
pub fn generate_default_env<A: 'static + Agent>(pop_size: u64) -> Result<Box<dyn Environment>, &'static str> {
    let mut pop: Vec<Box<dyn Agent>> = vec![];
    for _ in 0..pop_size {
        let agent: Box<dyn Agent> = A::generate()?;
        pop.push(agent);
    }
    let env: Box<DefaultEnvironment> = DefaultEnvironment::generate(pop)?;
    Ok(env)
}

/// Applies a tick to a passed in environment. This takes both
/// the default environment provided by this library and custom
/// defined environments created by the user.
pub fn tick(environment: &mut Box<dyn Environment>) -> Result<(), &'static str> {
    (*environment).tick()?;
    Ok(())
}

/// Applies a tick and a collent to a passed in environment.
/// This takes both the default environment provided by this
/// library and custom defined environments created by the user.
/// This function can be used when the user requies data from a
/// certain time in a running simulation.
pub fn tick_collect(environment: &mut Box<dyn Environment>) -> Result<(), &'static str> {
    (*environment).tick()?;
    (*environment).collect()?;
    Ok(())
}

/// Applies a collect to a passed in environment. This takes both
/// the default environment provided by this library and custom
/// defined environments created by the user.
pub fn collect(environment: Box<dyn Environment>) -> Result<(), &'static str> {
    (*environment).collect()?;
    Ok(())
}

/// Generates an environment and runs it the simulation in multiple
/// processes. This also runs the generated simulation with the
/// given parameters.
pub fn generate_default_tick_collect<A: 'static + Agent>(pop_size: u64, ticks: u64, runs: u64) -> Result<(), &'static str> {
    let cpu_count: u64 = num_cpus::get() as u64;
    for _ in 0..(runs / cpu_count + 1) {
        let mut v = vec![];
        for _ in 0..cpu_count {
            v.push(thread::spawn(move || -> Result<(), &'static str> {
                let mut env: Box<dyn Environment> = generate_default_env::<A>(pop_size)?;
                for _ in 0..ticks {
                    tick(&mut env)?;
                }
                collect(env)?;
                Ok(())
            }));
        }
        for handle in v {
            handle.join().unwrap().unwrap();
        }
    }
    Ok(())
}

/// Generates a custom environment specified agent. This environment
/// is the standard implementation and does not provide any custom
/// behavior.
pub fn generate_env<E: 'static + Environment, A: 'static + Agent>(pop_size: u64) -> Result<Box<dyn Environment>, &'static str> {
    let mut pop: Vec<Box<dyn Agent>> = vec![];
    for _ in 0..pop_size {
        let agent: Box<dyn Agent> = A::generate()?;
        pop.push(agent);
    }
    let env: Box<E> = E::generate(pop)?;
    Ok(env)
}

/// Generates a custom environment and runs it the simulation in
/// multiple processes. This also runs the generated simulation
/// with the given parameters.
pub fn generate_tick_collect<E: 'static + Environment, A: 'static + Agent>(pop_size: u64, ticks: u64, runs: u64) -> Result<(), &'static str> {
    let cpu_count: u64 = num_cpus::get() as u64;
    for _ in 0..(runs / cpu_count + 1) {
        let mut v = vec![];
        for _ in 0..cpu_count {
            v.push(thread::spawn(move || -> Result<(), &'static str> {
                let mut env: Box<dyn Environment> = generate_env::<E, A>(pop_size)?;
                for _ in 0..ticks {
                    tick(&mut env)?;
                }
                collect(env)?;
                Ok(())
            }));
        }
        for handle in v {
            handle.join().unwrap().unwrap();
        }
    }
    Ok(())
}

/// A default struct that is used in the default functions.
struct DefaultEnvironment {
    population: Vec<Box<dyn Agent>>,
}

impl Environment for DefaultEnvironment {
    fn generate(population: Vec<Box<dyn Agent>>) -> Result<Box<Self>, &'static str> {
        Ok(Box::new(Self { population }))
    }

    fn collect(&self) -> Result<(), &'static str> {
        for agent in &self.population {
            (*agent).collect()?;
        }
        Ok(())
    }

    fn tick(&mut self) -> Result<(), &'static str> {
        let mut pop: Vec<Box<dyn Agent>> = vec![];
        for _ in 0..self.population.len() {
            let mut agent: Box<dyn Agent> = self.population.pop().unwrap();
            agent.tick()?;
            pop.push(agent);
        }
        self.population = pop;
        Ok(())
    }

    fn add_agent(&mut self, agent: Box<dyn Agent>) -> Result<(), &'static str> {
        self.population.push(agent);
        Ok(())
    }

    fn add_agents(&mut self, agents: Vec<Box<dyn Agent>>) -> Result<(), &'static str> {
        for agent in agents {
            self.population.push(agent);
        }
        Ok(())
    }
}
