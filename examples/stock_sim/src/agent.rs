use config::{File, FileFormat};
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
    pub fn new(path: &str) -> Result<Box<Self>, Box<dyn std::error::Error>> {
        let config: Stock = config::Config::builder()
            .add_source(File::new(path, FileFormat::Toml))
            .build()?.try_deserialize::<Stock>()?;
        Ok(Box::new(config))
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
