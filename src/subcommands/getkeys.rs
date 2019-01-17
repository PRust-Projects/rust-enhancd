use crate::database::Database;
use clap::ArgMatches;

pub fn parse(db: &mut Database, _args: &ArgMatches) {
    if let Some(ordered_keys) = db.ordered_keys() {
        for path in &ordered_keys {
            println!("{}", path);
        }
    }
}
