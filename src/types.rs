use clap::ArgMatches;
use serde::{Serialize, Deserialize};

pub type Callback = fn(&ArgMatches);

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub email: String,
    pub alias: String
}