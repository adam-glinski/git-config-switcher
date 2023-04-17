use crate::types::{ConfigsMap};

use clap::ArgMatches;

pub fn on_list(
    sub_matches: &ArgMatches,
    configs_map: &mut ConfigsMap,
)