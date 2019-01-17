use crate::database::Database;
use clap::ArgMatches;

pub fn parse(db: &mut Database, args: &ArgMatches) {
    let path = args.value_of("path").expect("Missing path argument!");
    let str_weight = args.value_of("weight").expect("Missing weight argument!");
    let weight = str_weight
        .parse()
        .expect("The weight should be an integer!");

    if !db.insert(path.to_string(), weight) {
        panic!("The path was not inserted successfully!");
    }
}
