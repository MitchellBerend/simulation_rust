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
use color_eyre::Report;

#[allow(unused_imports)]
use crate::traits::{Environment, Agent};

#[allow(dead_code)]
fn create_agent<T: 'static + Agent>() -> Result<Box<dyn Agent>, Report> {
    Ok(Box::new(T::generate()?))
}


pub fn generate_env<E: 'static + Environment, A: 'static + Agent>(population_size: u16) -> Result<Box<E>, Report> {
    let mut pop: Vec<A> = vec!();
    for _ in 0..population_size {
        let agent = A::generate()?;
        pop.push(agent);
    }

    let env = E::generate::<A>(pop)?;
    Ok(Box::new(env))
}


#[allow(dead_code)]
pub fn tick<E: Environment, A: Agent>(environment: E) -> Result<E, Report> {
    let mut pop: Vec<A> = vec!();
    for _ in 0..environment.len() {
        let mut agent: A = environment.pop()?;
        agent.tick()?;
        pop.push(agent);
    }
    Ok(E::generate::<A>(pop)?)
}


#[allow(dead_code)]
pub fn tick_collect<E: Environment, A: Agent>(environment: E) -> Result<E, Report> {
    let mut pop: Vec<A> = vec!();
    for _ in 0..environment.len() {
        let mut agent: A = environment.pop()?;
        agent.tick()?;
        agent.collect()?;
        pop.push(agent);
    }
    Ok(E::generate::<A>(pop)?)
}




/*
#[cfg(test)]
mod tests {
    use crate::{Agent};
    use crate::functions::*;

    struct TestAgent {
        pub age: u8,
    }
    
    struct TestEnv {
        year: u16,
        pub population: Vec<Box<dyn Agent>>,
    }
 
    impl Environment for TestEnv {
        fn generate<T: Agent>() -> Result<TestEnv, Report> {
            let mut rv = TestEnv {
                year: 0,
                population: vec!()
            };
            for _ in 1..10 {
                let t: Box<dyn Agent> = TestAgent::generate()?;
                rv.population.push(t);
            }
            Ok(rv)
        }

        fn collect(&self) -> Result<(), Report> {
            for agent in self.population()? {
                agent.collect();
            }
            Ok(())
        }

        fn tick(&self) -> Result<(), Report> {
            for mut agent in self.population()? {
                agent.tick();
            }
            Ok(())
        }

        fn population(self) -> Result<Vec<Box<dyn Agent>>, Report> {
            Ok(self.population)
        }
    }

    impl Agent for TestAgent {
        fn generate() -> Result<TestAgent, Report> {
            Ok(TestAgent {age: 0})
        }

        fn collect(&self) -> Result<(), Report> {
            println!("{}", &self.age);
            Ok(())
        }

        fn tick(&mut self) -> Result<(), Report> {
            self.age += 1;
            Ok(())
        }
    }


    #[test]
    fn it_works() {
        let mut env: Vec<TestAgent> = vec!();
        for _ in 1..100 {
            let agent = TestAgent::generate().unwrap();
            &env.push(agent);
        }
        for _ in 1..10 {
            let mut _env: Vec<TestAgent> = vec!();
            for _ in 0..env.len()-1 {
                if let Some(mut agent) = env.pop() {
                    agent.tick().unwrap();
                    agent.collect().unwrap();
                    _env.push(agent)
                }
            }
            env = _env;
        }
        println!("End of program");
    }
}
*/
