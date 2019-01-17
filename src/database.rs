use regex::Regex;
use rustc_hash::FxHashMap;
use serde_yaml;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Write};

#[derive(Debug)]
struct DatabaseEntry {
    key: String,
    value: u32,
}

#[derive(Debug)]
pub struct Database<'a> {
    filename: &'a str,
    data: FxHashMap<String, u32>,
}

impl<'b> Database<'b> {
    pub fn new(filename: &'b str) -> Result<Database, Error> {
        match File::open(filename) {
            Ok(mut file) => {
                let mut file_content = String::new();
                file.read_to_string(&mut file_content)?;
                let data = serde_yaml::from_str(&file_content)
                    .map_err(|e| Error::new(ErrorKind::Other, e))?;
                Ok(Database { filename, data })
            }
            Err(_) => Ok(Database {
                filename,
                data: FxHashMap::default(),
            }),
        }
    }

    pub fn get(&self, path: &'b str) -> Option<&u32> {
        self.data.get(path)
    }

    pub fn insert(&mut self, path: String, weight: u32) -> bool {
        match self.data.get(&path) {
            Some(_) => false,
            None => {
                self.data.insert(path, weight);
                true
            }
        }
    }

    pub fn update(&mut self, path: String, weight: u32) -> bool {
        match self.data.get(&path) {
            Some(_) => {
                self.data.insert(path, weight);
                true
            }
            None => false,
        }
    }

    pub fn delete(&mut self, path: &str) -> bool {
        self.data.remove(path).is_some()
    }

    pub fn ordered_keys(&self) -> Option<Vec<String>> {
        let mut unordered_entries = Vec::with_capacity(self.data.len());
        for (key, val) in self.data.iter() {
            unordered_entries.push(DatabaseEntry {
                key: key.to_string(),
                value: *val,
            });
        }

        if let Ok(hidden_files_re) = Regex::new(r"/\.") {
            unordered_entries.as_mut_slice().sort_by(|a, b| {
                if a.value > b.value {
                    return Ordering::Less;
                } else if a.value < b.value {
                    return Ordering::Greater;
                } else {
                    let a_has_hidden = hidden_files_re.is_match(&a.key);
                    let b_has_hidden = hidden_files_re.is_match(&b.key);
                    if a_has_hidden && b_has_hidden {
                        return (a.key).cmp(&b.key);
                    } else if a_has_hidden {
                        return Ordering::Greater;
                    } else if b_has_hidden {
                        return Ordering::Less;
                    } else {
                        return (a.key).cmp(&b.key);
                    }
                }
            });

            let mut ordered_keys = Vec::with_capacity(unordered_entries.len());
            for entry in &unordered_entries {
                ordered_keys.push(entry.key.clone());
            }
            return Some(ordered_keys);
        }

        None
    }

    pub fn save(&self) -> Result<bool, Error> {
        let file_content =
            serde_yaml::to_string(&self.data).map_err(|e| Error::new(ErrorKind::Other, e))?;
        let mut file = File::create(self.filename)?;
        file.write_all(file_content.as_bytes())?;
        Ok(true)
    }
}

#[cfg(test)]
mod tests {

    use crate::database::Database;
    use std::fs::{copy, File, remove_file};
    use std::io::Read;

    fn must_copy(source: &str, dest: &str) {
        if let Err(_) = copy(source, dest) {
            panic!("File did not copy successfully!");
        }
    }

    #[test]
    fn nonexistent_database_test() {
        let _ = remove_file("test/nonexistent.db");

        let db = Database::new("test/nonexistent.db").unwrap();
        let ordered_keys = db.ordered_keys().unwrap();
        assert_eq!(ordered_keys.len(), 0, "Expected the database to be empty!");

        let _ = remove_file("test/nonexistent_test.db");
    }

    #[test]
    fn existent_database_test() {
        must_copy("test/test.db", "test/existent_test.db");

        let db = Database::new("test/existent_test.db").unwrap();
        let ordered_keys = db.ordered_keys().unwrap();
        assert_eq!(
            ordered_keys.len(),
            10,
            "Expected the database to contain ten entries!"
        );

        let _ = remove_file("test/existent_test.db");
    }

    #[test]
    fn get_from_database_test() {
        must_copy("test/test.db", "test/get_test.db");

        let db = Database::new("test/get_test.db").unwrap();
        assert_eq!(
            db.get("Code/rust"),
            Some(&3),
            "Expected the weight to be 3!"
        );
        assert_eq!(
            db.get("Code/go"),
            None,
            "Expected None as the key does not exist!"
        );

        let _ = remove_file("test/get_test.db");
    }

    #[test]
    fn insert_into_database_test() {
        must_copy("test/test.db", "test/insert_test.db");

        let mut db = Database::new("test/insert_test.db").unwrap();
        assert!(db.insert(String::from("code/rust"), 3));
        assert!(!db.insert(String::from("Code/rust"), 3));

        let _ = remove_file("test/insert_test.db");
    }

    #[test]
    fn update_database_test() {
        must_copy("test/test.db", "test/update_test.db");

        let mut db = Database::new("test/update_test.db").unwrap();
        assert!(db.update(String::from("Code/.hello"), 4));
        assert!(!db.update(String::from("Code/go"), 4));

        let _ = remove_file("test/update_test.db");
    }

    #[test]
    fn delete_from_database_test() {
        must_copy("test/test.db", "test/delete_test.db");

        let mut db = Database::new("test/delete_test.db").unwrap();
        assert!(db.delete("Code/.hello"));
        assert!(!db.delete("Code/go"));

        let ordered_keys = db.ordered_keys().unwrap();
        assert_eq!(
            ordered_keys.len(),
            9,
            "Expected database to only contain 9 entries now!"
        );
        assert_eq!(
            db.get("Code/.hello"),
            None,
            "Expected the entry to be deleted!"
        );

        let _ = remove_file("test/delete_test.db");
    }

    #[test]
    fn ordered_keys_test() {
        must_copy("test/test.db", "test/ordered_keys_test.db");

        let expected_ordered_keys = [
            "Code/.hi",
            "Code/he.lo",
            "Code/helo",
            "Code/death",
            "Code/h.ello",
            "Code/hi/die/die/die/s",
            "Code/rust",
            "Code/.death",
            "Code/.hello",
            "Code/hi/sie/die/as/.asdf/3232/3",
        ];

        let db = Database::new("test/ordered_keys_test.db").unwrap();
        let ordered_keys = db.ordered_keys().unwrap();
        assert_eq!(ordered_keys.len(), 10, "Expected there to be 10 entries!");
        assert_eq!(
            ordered_keys, expected_ordered_keys,
            "Expect the two set of ordered keys to be equal!"
        );

        let _ = remove_file("test/ordered_keys_test.db");
    }

    #[test]
    fn save_database_test() {
        let mut db = Database::new("test/save_test.db").unwrap();
        assert!(db.insert(String::from("Hello"), 3));
        assert!(db.insert(String::from("Hello1"), 4));
        assert!(db.save().is_ok(), "Expected the file to save successfully!");

        let expected_content = "---\nHello: 3\nHello1: 4";
        let mut file_content = String::new();
        let mut file = File::open("test/save_test.db").unwrap();
        let _ = file.read_to_string(&mut file_content);
        assert_eq!(file_content, expected_content, "Expected the database to be saved correctly!");
        
        let _ = remove_file("test/save_test.db");
    }

}
