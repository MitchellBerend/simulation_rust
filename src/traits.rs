//! This module holds all the traits that have to be implemented for 
//! the agent to be a plugable compontent.
//! 
//! The following mental model is used to the api design.
//!
//! There will be constructor functions that construct and run the simulation automatically. These
//! functions can take in any abritrary type that has implemented the correct traits. This makes it
//! so multiple types can be defined as agents and a simulation can be run in the same manner as
//! with only a single agent type.
//!
//! The point of the simulations is to add some form of random sequencing. This means there will
//! need to be multiple runs of the simulation with the same input parameters. One simulation run 
//! will be fully separate of another simulation run, making a multiprocess execution model a very 
//! good candidate for speeding up the program. Once all runs are finished and writen to their own
//! results file. A clean up function can collect and merge all the data into a single file.
//!
//! The process will look something like this:
//! 
//!
//! /*                  Environment
//!                         |
//!                 --------------------
//!                 |       |           |
//!             [Agent_1,   |           |
//!                         Agent_2,    |
//!                                     Agent_3, ... ]
//! */
//!
//! The environment will give out a tick command that propagates down to all the agents contained
//! in its system.
//!
//! /*                Environment.tick()
//!                         |
//!                 --------------------
//!                 |       |           |
//!     [Agent_1.tick()     |           |
//!                     Agent_2.tick()  |
//!                                 Agent_3.tick() ]
//! */
//!
//! Once that round is completely finished the environment will collect all the public data and
//! write it to a file in json format.
//!
//!
//! /*                  Environment.collect()
//!                         |
//!                 --------------------
//!                 |       |           |
//!     [Agent_1.collect()  |           |
//!                     Agent_2.collect()
//!                                 Agent_3.collect() ]
//! */
//! An Agent needs to have the following fields:
//!     -
//!
//! An Environment needs to have the following fields: 
//!     - population


use color_eyre::Report;


pub trait Agent {
    ///
    fn generate() -> Result<Box<Self>, Report> where Self: Sized;

    ///
    fn collect(&self) -> Result<(), Report>;

    ///
    fn tick(&mut self) -> Result<(), Report>;
}


pub trait Environment {
    ///
    fn generate(pop: Vec<Box<dyn Agent>>) -> Result<Box<Self>, Report> where Self: Sized;

    ///
    fn collect(&self) -> Result<(), Report>;

    ///
    fn tick(&mut self) -> Result<(), Report>;
}

