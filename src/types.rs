use std::collections::HashMap;

use clap::ArgMatches;
use serde::{Serialize, Deserialize};
use serde_json::error::Result;

pub type AddCallback = fn(&ArgMatches, &mut HashMap<String, Config>) -> Result<()>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub email: String
}
