use std::collections::HashMap;

use clap::ArgMatches;
use serde::{Serialize, Deserialize};
use serde_json::error::Result;

pub type ConfigsMap = HashMap<String, Config>;
pub type Callback = fn(&ArgMatches, &mut ConfigsMap) -> Result<()>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub email: String
}
