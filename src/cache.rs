
use std::path::PathBuf;

lazy_static! {
    static ref CACHE_DATA_PATH: PathBuf = {
        let loc: PathBuf = PathBuf::from(env!("AC_DATABASE"));
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
    pub fid: u8,
    pub group: Vec<u8>,
    pub other_args: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cache {
    pub major_version: u8,
    pub minor_version: u8,
    pub map: HashMap<CacheEntry, u8>,
}

lazy_static! {
    static ref DATABASE: Mutex<FileDatabase<Cache, Bincode>> = {
        let fd = FileDatabase::from_path(&*CACHE_DATA_PATH.clone(), 
            Cache {
                major_version: 0u8,
                minor_version: 1u8,
                map: HashMap::new(),
            })
            .expect("Error opening cache database file");
        fd.load().unwrap_or_else(|_| {
            fd.save().expect("Error initializing empty database file");  
        });
        Mutex::new(fd)
    };
}

pub fn cache_get(cache_entry: &CacheEntry) -> Option<u32> {
    let mut result: Option<u32> = None;
    let db = DATABASE.lock().unwrap();
    db.read(|db| {
        let r = db.map.get(cache_entry);
        if let Some(&val) = r {
            result = Some(val as u32);
        }
    }).unwrap();
    result
}

pub fn cache_set(cache_entry: CacheEntry, v: u8) {
    let db = DATABASE.lock().unwrap();
    db.write(|db| {
        db.map.insert(cache_entry, v);
    }).expect("Error writing to cache database file!");
    db.save().expect("Error saving to cache database file!");
}
