use crate::database::Database;
use clap::ArgMatches;

pub fn parse(db: &mut Database, args: &ArgMatches) {
    let path = args.value_of("path").expect("Missing path argument!");
    if let Some(weight) = db.get(path) {
        if !db.update(path.to_string(), *weight + 1) {
            panic!("The path was not updated successfully!");
        }
    } else {
        panic!("The path cannot be found!");
    }
}
