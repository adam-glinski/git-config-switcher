use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub type ConfigsMap = HashMap<String, Config>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub email: String
}
