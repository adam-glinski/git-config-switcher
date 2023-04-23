use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type ConfigsMap = HashMap<String, Config>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub email: String,
}
