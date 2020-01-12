
#[macro_use]
extern crate lazy_static;
extern crate rustbreak;

use std::path::PathBuf;
use std::path::Path;
use std::fmt::Debug;
use std::env;
use std::fs;
use std::io;

lazy_static! {
    pub static ref CACHE_DATA_PATH: PathBuf = {
        if let Ok(home) = env::var("HOME") {
            let mut acpath = PathBuf::from(&home);
            acpath.push(".addcomb");
            fs::create_dir(&acpath)
                .unwrap_or_else(|err| {
                    match err.kind() {
                        io::ErrorKind::AlreadyExists => (),
                        e => panic!("error creating .addcomb directory in HOME: {:?}", e)
                    }
                });
            acpath.push("cache");
            acpath.set_extension("bincode");
            acpath
        } else {
            panic!("Unable to find $HOME to find cache database");
        }
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

pub fn db_from_path<S>(path: S) -> FileDatabase<Cache, Bincode>
    where S: AsRef<Path> + Debug
{
    let fd = FileDatabase::from_path(&path, Cache { major_version: 0u8, minor_version: 1u8, map: HashMap::new() })
        .unwrap_or_else(|e| panic!("Error opening FileDatabase from path {:?}: {}", &path, e));
    fd.load().unwrap_or_else(|e| panic!("Error reading FileDatabase from path {:?}: {}", &path, e));
    fd
}

lazy_static! {
    pub static ref DEFAULT_DATABASE: Mutex<FileDatabase<Cache, Bincode>> = {
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
    let db = DEFAULT_DATABASE.lock().unwrap();
    db.read(|db| {
        let r = db.map.get(cache_entry);
        if let Some(&val) = r {
            result = Some(val as u32);
        }
    }).unwrap();
    result
}

pub fn cache_set(cache_entry: CacheEntry, v: u8) {
    let db = DEFAULT_DATABASE.lock().unwrap();
    db.write(|db| {
        db.map.insert(cache_entry, v);
    }).expect("Error writing to cache database file!");
    db.save().expect("Error saving to cache database file!");
}
