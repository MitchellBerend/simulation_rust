use serde::{Deserialize, Serialize};
use sim_rust::Agent;

use rand_distr::Distribution;

#[derive(Debug, Serialize, Deserialize)]
pub struct Stock {
    name: String,
    price: f32,
    mean: f32,
    std_dev: f32,
}

impl Stock {
    pub fn new(path: &str) -> Result<Box<Self>, &'static str> {
        let mut config = config::Config::default();
        if let Err(_) = config.merge(config::File::with_name(path)) {
            panic!("Error while trying to load {}", path);
        }
        let conf  = match config.try_into::<Stock>() {
            Ok(conf) => Box::new(conf),
            Err(_) => panic!("Could not build Stock instance for {}", path),
        };
        Ok(conf)
    }
}

impl Agent for Stock {
    fn generate() -> Result<Box<Self>, &'static str> {
        unimplemented!()
    }

    fn tick(&mut self) -> Result<(), &'static str> {
        let dist = match rand_distr::Normal::new(self.mean, self.std_dev) {
            Ok(dist) => dist,
            Err(_) => panic!("Could not generate a distribution"),
        };
        let delta = dist.sample(&mut rand::thread_rng());
        self.price *= delta;
        Ok(())
    }

    fn collect(&self) -> Result<(), &'static str> {
        println!("\t{{name: \"{}\",price: {}}}", &self.name, &self.price);
        Ok(())
    }
}
