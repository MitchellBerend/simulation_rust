use simulation_rust::Agent;
use simulation_rust::Environment;
use simulation_rust::functions::*;

use color_eyre::Report;


struct TestAgent {
    pub age: u64,
}

struct TestEnv {
    pub population: Vec<Box<dyn Agent>>,
}

impl Environment for TestEnv {
    fn generate(pop: Vec<Box<dyn Agent>>) -> Result<Box<TestEnv>, Report> {
        Ok(Box::new(TestEnv { population: pop }))
    }

    fn collect(&self) -> Result<(), Report> {
        for agent in &self.population {
            (*agent).collect()?;
        }
        Ok(())
    }

    fn tick(&mut self) -> Result<(), Report> {
        let mut pop: Vec<Box<dyn Agent>> = vec!();
        for _ in 0..self.population.len() {
            let mut agent: Box<dyn Agent> = self.population.pop().unwrap();
            agent.tick()?;
            pop.push(agent);
        }
        self.population = pop;
        Ok(())
    }
}

impl Agent for TestAgent {
    fn generate() -> Result<Box<Self>, Report> {
        Ok(Box::new(TestAgent { age: 0 }))
    }

    fn collect(&self) -> Result<(), Report> {
        Ok(())
    }

    fn tick(&mut self) -> Result<(), Report> {
        self.age += 1;
        Ok(())
    }
}


#[test]
fn test_tick() -> Result<(), Report> {
    let mut env = generate_env::<TestEnv, TestAgent>(10)?;
    for _ in 0..100 {
        env = tick(env)?;
    }
    Ok(())
}


#[test]
fn test_tick_collect_once() -> Result<(), Report> {
    let mut env = generate_env::<TestEnv, TestAgent>(1000)?;
    for _ in 0..100 {
        env = tick(env)?;
    }
    collect(env)?;
    Ok(())
}


#[test]
fn test_multithread_tick_collect_once() -> Result<(), Report> {
    generate_tick_collect::<TestEnv, TestAgent>(10000, 10000, 100)?;
    Ok(())
}
