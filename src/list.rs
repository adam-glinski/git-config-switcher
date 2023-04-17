use prettytable::{Table, row};

use crate::types::{ConfigsMap};

pub fn on_list(configs_map: &mut ConfigsMap) {
    let mut table = Table::new();
    table.add_row(row![bFg->"Config alias", bFg->"Author name", bFg->"Author email"]);

    // println!("alias\tname\temail");
    for config_alias in configs_map.keys() {
        let config = configs_map.get(config_alias).unwrap();
        // println!("{}\t{}\t{}", config_alias, config.name, config.email);
        table.add_row(row![config_alias, config.name, config.email]);
    }

    table.printstd();
}