use crate::database::Database;
use clap::ArgMatches;

pub fn parse(db: &mut Database, args: &ArgMatches) {
    let path = args.value_of("path").expect("Missing path argument!");
    if !db.delete(path) {
        panic!("The path was not deleted successfully!");
    }
}
