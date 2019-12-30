
use std::path::PathBuf;
use std::env;

lazy_static! {
    static ref CACHE_DATA_PATH: PathBuf = {
        let mut loc: PathBuf = env::current_exe().expect("Cannot find the path of the installed addcomb executable; this is needed to setup the cache file.");
        loc.pop(); // Directory of .so file (directory in site-packages)
        loc.push("cache");
        loc.set_extension("bincode");
        loc
    };
}

use std::collections::HashMap;
use rustbreak::{FileDatabase, deser::Bincode};
use serde::Serialize;
use serde::Deserialize;

use std::sync::Mutex;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CacheEntry {
    pub fname: String,
    pub group: Vec<u32>,
    pub other_args: Vec<u32>,
}

lazy_static! {
    static ref DATABASE: Mutex<FileDatabase<HashMap<CacheEntry, u32>, Bincode>> = {
        let fd = FileDatabase::from_path(&*CACHE_DATA_PATH.clone(), HashMap::new());
        Mutex::new(fd.expect("Error opening cache database file"))
    };
}

pub fn cache_get(cache_entry: &CacheEntry) -> Option<&u32> {
    let mut result: Option<u32> = None;
    let db = DATABASE.lock().unwrap();
    db.read(|db| {
        let r = db.get(cache_entry);
        if let Some(&val) = r {
            result = Some(val);
        }
    }).unwrap();
    result
}

pub fn cache_set(cache_entry: &CacheEntry, v: u32) {
    let db = DATABASE.lock().unwrap();
    db.write(|db| {
        db.insert(cache_entry, v);
    }).expect("Error writing to cache database file!");
    db.save().expect("Error saving to cache database file!");
}
