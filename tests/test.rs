use sim_rust::functions::*;
use sim_rust::Agent;

struct TestAgent {
    pub age: u64,
}

impl Agent for TestAgent {
    fn generate() -> Result<Box<Self>, &'static str> {
        Ok(Box::new(TestAgent { age: 0 }))
    }

    fn collect(&self) -> Result<(), &'static str> {
        Ok(())
    }

    fn tick(&mut self) -> Result<(), &'static str> {
        self.age += 1;
        Ok(())
    }

    fn clean(&self) -> Result<(), &'static str> {
        drop(self);
        Ok(())
    }
}

impl Drop for TestAgent {
    fn drop(&mut self) {
        println!("Dropping agent")
    }
}

#[test]
fn test_tick() -> Result<(), &'static str> {
    let mut env = generate_default_env::<TestAgent>(10)?;
    let new_agent = TestAgent::generate()?;
    env.add_agent(new_agent)?;
    let new_agents: Vec<Box<dyn Agent>> = vec![
        TestAgent::generate()?,
        TestAgent::generate()?,
        TestAgent::generate()?,
    ];
    env.add_agents(new_agents)?;
    for _ in 0..100 {
        tick(&mut env)?;
    }
    Ok(())
}

#[test]
fn test_tick_collect_once() -> Result<(), &'static str> {
    let mut env = generate_default_env::<TestAgent>(1000)?;
    let new_agent = TestAgent::generate()?;
    env.add_agent(new_agent)?;
    let new_agents: Vec<Box<dyn Agent>> = vec![
        TestAgent::generate()?,
        TestAgent::generate()?,
        TestAgent::generate()?,
    ];
    env.add_agents(new_agents)?;
    for _ in 0..100 {
        tick(&mut env)?;
    }
    collect(env)?;
    Ok(())
}

#[test]
fn test_multithread_tick_collect_once() -> Result<(), &'static str> {
    generate_default_tick_collect::<TestAgent>(10000, 10000, 100)?;
    Ok(())
}
