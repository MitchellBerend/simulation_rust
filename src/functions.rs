//! This module will hold all the main functions that should be called. The specific order is
//! recommended but some functions depend on the build up or tear down of others.
//!
//!
//!


#[allow(unused_imports)]
use std::process;
#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::io::prelude::*;

#[allow(unused_imports)]
use color_eyre::{Report, eyre::eyre} ;

#[allow(unused_imports)]
use crate::traits::{Environment, Agent};

#[allow(dead_code)]
fn create_agent<T: 'static + Agent>() -> Result<Box<dyn Agent>, Report> {
    Ok(Box::new(T::generate()?))
}


pub fn generate_env<E: 'static + Environment, A: 'static + Agent>(population_size: u16) -> Result<Box<E>, Report> {
    let mut pop: Vec<Box<A>> = vec!();
    for _ in 0..population_size {
        let agent: Box<A> = Box::new(A::generate()?);
        pop.push(agent);
    }
    let env = E::generate::<A>(pop)?;
    Ok(Box::new(env))
}


#[allow(dead_code)]
pub fn tick<E: Environment, A: Agent>(environment: E) -> Result<E, Report> {
    let mut pop: Vec<Box<A>> = vec!();
    for _ in 0..environment.len() {
        let mut agent: Box<A> = environment.pop()?;
        agent.tick()?;
        pop.push(agent);
    }
    Ok(E::generate::<A>(pop)?)
}

/*
#[allow(dead_code)]
pub fn tick_collect<E: Environment, A: Agent>(environment: E) -> Result<E, Report> {
    let mut pop: Vec<A> = vec!();
    for _ in 0..environment.len() {
        let mut agent: A = environment.pop()?;
        agent.tick()?;
        agent.collect()?;
        pop.push(Box::new(agent));
    }
    Ok(E::generate::<A>(pop)?)
}
*/




#[cfg(test)]
mod tests {
    use crate::{Agent, Environment};
    use crate::functions::*;

    struct TestAgent {
        pub age: u8,
    }
    
    struct TestEnv {
        population: Vec<Box<TestAgent>>,
    }

    impl Environment for TestEnv {
        fn generate<A: Agent>(pop: Vec<Box<A>>) -> Result<TestEnv, Report> {
            Ok(TestEnv { population: pop })
        }

        fn collect(&self) -> Result<(), Report> {
            todo!()
        }

        fn tick(&self) -> Result<(), Report> {
            todo!()
        }

        fn pop<A: Agent>(&self) -> Result<Box<A>, Report> {
            let agent: Box<A>  = self.population.pop().unwrap();
            Ok(agent)
        }

        fn len(&self) -> usize {
            return self.population.len()
        }
    }

    impl Agent for TestAgent {
        fn generate() -> Result<Self, Report> {
            todo!()
        }

        fn collect(&self) -> Result<(), Report> {
            todo!()
        }

        fn tick(&mut self) -> Result<(), Report> {
            todo!()
        }
    }


    #[test]
    fn it_works() {
        let env = generate_env::<TestEnv, TestAgent>(100);        
    }
}
