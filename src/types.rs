use clap::ArgMatches;
use serde::{Serialize, Deserialize};
use serde_json::Result;

pub type Callback = fn(&ArgMatches) -> Result<()> ;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub email: String
}
