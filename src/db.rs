use serde;
use serde::Deserialize;
use serde::Serialize;
use std::fs;

// Custom Error Enum
#[derive(Debug)]
pub enum DBError {
    EntryNotFound,
    _Unknown,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Database {
    data: Vec<DBNode>,
}
impl Database {
    pub fn new() -> Database {
        let empty = Vec::new();

        Database { data: empty }
    }

    pub fn open(&mut self, path: &str) -> Self {
        if let Ok(x) = fs::read_to_string(path) {
            let mut x: Database = serde_json::from_str(&x).expect("Cannot deserialise database!");

            self.data.append(&mut x.data);

            self.clone()

        // TODO change this into json deserialization
        // if let Ok(data) = toml::from_str(&x) {
        //     println!("Data read Okay!!");
        //     return data;
        // } else {
        //     println!("Data didnt do Okay!!");
        //     let empty = Vec::new();
        //
        //     return Database(empty);
        // }
        } else {
            let empty = Vec::new();

            return Database { data: empty };
        }
    }

    pub fn insert(&mut self, value: DBValue) -> &mut Self {
        if let None = self.data.last() {
            self.data.push(DBNode { key: 0, value })
        } else {
            let last_index = self.data.last().unwrap().key;

            self.data.push(DBNode {
                key: last_index + 1,
                value,
            })
        }

        self
    }

    pub fn delete(&mut self, key: usize) -> Result<&mut Self, DBError> {
        if let Some(entry) = self.data.iter().enumerate().find(|x| x.1.key == key) {
            println!("Found this value: {:?}", entry);
            self.data.remove(entry.0);
        } else {
            //need to break in here
            return Err(DBError::EntryNotFound);
        }

        Ok(self)
    }

    pub fn search_by_val(&self, value: DBValue) -> Result<DBValue, DBError> {
        if let Some(entry) = self.data.iter().find(|x| x.value == value) {
            return Ok(entry.value.clone());
        } else {
            return Err(DBError::EntryNotFound);
        }
    }

    pub fn search_by_key(&self, key: usize) -> Result<DBValue, DBError> {
        if let Some(entry) = self.data.iter().find(|x| x.key == key) {
            return Ok(entry.value.clone());
        } else {
            return Err(DBError::EntryNotFound);
        }
    }

    pub fn update(&mut self, key: usize, value: DBValue) -> Result<&mut Self, DBError> {
        if let Some(_) = self.data.iter().find(|x| x.key == key) {
            self.data[key].value = value;

            Ok(self)
        } else {
            return Err(DBError::EntryNotFound);
        }
    }

    pub fn save_db(&self, db_path: &str) {
        let serialised = serde_json::to_string(&self).expect("Cannot Serialise data...");

        fs::write(db_path, serialised).expect("Cannot write to this file!");
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct DBNode {
    pub key: usize,
    pub value: DBValue,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DBValue {
    pub entry: String,
}
impl DBValue {
    fn _new() -> Self {
        Self { entry: "".into() }
    }
}
impl PartialEq for DBValue {
    fn eq(&self, other: &Self) -> bool {
        if self.entry == other.entry {
            return true;
        }
        return false;
    }
}
