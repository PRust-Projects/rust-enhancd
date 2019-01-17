mod database;
mod renhancd_error;
mod subcommands;

use crate::database::Database;
use crate::renhancd_error::{error_and_exit, ErrorExt};
use clap::{clap_app, ArgMatches};
use dirs::{config_dir, home_dir};
use file_walkers::parallel;
use std::fs::create_dir;

fn parse_subcommands(main_args: ArgMatches) {
    if main_args.subcommand_name().is_none() {
        println!("This app allows you to persistently store a list of directories weighted based");
        println!("on access frequency.\n");
        println!("Run `rust-enhancd help` for more information!");
    }

    let mut configdir =
        config_dir().unwrap_or_error_and_exit("Cannot retrieve the config directory");
    configdir.push("rust_enhancd");
    create_dir(&configdir).unwrap_or_error_and_exit(&format!(
        "Cannot create the directory: {}",
        configdir.to_str().unwrap()
    ));

    configdir.push("renhancd.db");
    let mut db = Database::new(configdir.to_str().unwrap())
        .unwrap_or_error_and_exit("There was an error loading the database!");
    update_database(&mut db);

    if let Some(args) = main_args.subcommand_matches("insert") {
        subcommands::insert::parse(&mut db, args);
    }
    if let Some(args) = main_args.subcommand_matches("update") {
        subcommands::update::parse(&mut db, args);
    }
    if let Some(args) = main_args.subcommand_matches("delete") {
        subcommands::delete::parse(&mut db, args);
    }
    if let Some(args) = main_args.subcommand_matches("getkeys") {
        subcommands::getkeys::parse(&mut db, args);
    }

    db.save().exit_if_err("Unable to save the database!");
}

fn update_database(db: &mut Database) {
    if let Some(ordered_keys) = db.ordered_keys() {
        let homedir = home_dir().unwrap_or_error_and_exit("Cannot retrieve the home directory!");

        let directories = parallel::get_directories(homedir.to_str().unwrap(), false);
        for key in &ordered_keys {
            if !directories.contains(key) {
                if !db.delete(key) {
                    error_and_exit("Unable to update the database successfully!");
                }
            }
        }

        for dir in &directories {
            let _ = db.insert(dir.to_string(), 0);
        }

        return;
    }
}

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
    parse_subcommands(matches);
}
