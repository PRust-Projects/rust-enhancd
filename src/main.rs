mod database;

use crate::database::Database;
use clap::clap_app;

fn main() {
    let matches = clap_app!(rust_enhancd =>
                            (version: "1.0")
                            (author: "PChan")
                            (about: "Keep a persistence database of which directories are frequently accessed")
                            (@subcommand insert =>
                             (about: "Insert an entry into the database")
                             (@arg path: +required "The path to insert")
                             (@arg weight: +required "The weight associated with the path, use 0 for new entries"))
                            (@subcommand update =>
                             (about: "Increment the weight of an entry in the database by 1")
                             (@arg path: +required "The path to update"))
                            (@subcommand delete =>
                             (about: "Delete an entry from the database")
                             (@arg path: +required "The path to delete"))
                            (@subcommand getkeys =>
                             (about: "Get all the path from the database ordered by weight"))).get_matches();
    if matches.subcommand_name().is_none() {
        println!("This app allows you to persistently store a list of directories weighted based on access frequency.\n");
        println!("Run `rust-enhancd help` for more information!");
        
    }

    let mut db = Database::new("renhancd.db").expect("There was an error loading the database!");

    if let Some(args) = matches.subcommand_matches("insert") {
        let path = args.value_of("path").unwrap();
        let weight = args.value_of("weight").unwrap();
        if !db.insert(path.to_string(), weight.parse().expect("The weight should be an integer!")) {
            panic!("The path was not inserted successfully!");
        }
    }
    if let Some(args) = matches.subcommand_matches("update") {
        let path = args.value_of("path").unwrap();
        if let Some(weight) = db.get(path) {
            if !db.update(path.to_string(), *weight + 1) {
                panic!("The path was not updated successfully!");
            }
        } else {
            panic!("The path cannot be found!");
        }
    }
    if let Some(args) = matches.subcommand_matches("delete") {
        let path = args.value_of("path").unwrap();
        if !db.delete(path) {
            panic!("The path was not deleted successfully!");
        }
    }
    if let Some(_) = matches.subcommand_matches("getkeys") {
        if let Some(ordered_keys) = db.ordered_keys() {
            for path in &ordered_keys {
                println!("{}", path);
            }
        }
    }

    if let Err(e) = db.save() {
        panic!(e);
    }
}
