use simulation_rust::Agent;
use simulation_rust::Environment;
use simulation_rust::functions::*;

use color_eyre::Report;


struct TestAgent {
    pub age: u8,
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
        println!("age: {}", &self.age);
        Ok(())
    }

    fn tick(&mut self) -> Result<(), Report> {
        self.age += 1;
        Ok(())
    }
}


#[test]
fn it_works() -> Result<(), Report>{
    let mut env = generate_env::<TestEnv, TestAgent>(10)?;
    for year in 0..100 {
        println!("Year: {}", year);
        env.tick()?;
        env.collect()?;
    }
    Ok(())
}