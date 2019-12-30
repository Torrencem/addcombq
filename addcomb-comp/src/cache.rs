
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
use cached::Cached;
use serde::Serialize;
use serde::Deserialize;

use std::sync::Mutex;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CacheEntry {
    fname: String,
    group: Vec<u32>,
    other_args: Vec<u32>,
}

lazy_static! {
    static ref DATABASE: Mutex<FileDatabase<HashMap<CacheEntry, u32>, Bincode>> = {
        let fd = FileDatabase::from_path(&*CACHE_DATA_PATH.clone(), HashMap::new());
        Mutex::new(fd.expect("Error opening cache database file"))
    };
}

struct BFCache {
    fname: String,
    cbuf: Vec<u32>,
}

impl Cached<(u32, u32, u32), u32> for BFCache {
    fn cache_get(&mut self, k: &(u32, u32, u32)) -> Option<&u32> {
        let cache_entry = CacheEntry {
            fname: self.fname.clone(),
            group: vec![k.0],
            other_args: vec![k.1, k.2],
        };
        let mut result: Option<u32> = None;
        let db = DATABASE.lock().unwrap();
        db.read(|db| {
            let r = db.get(&cache_entry);
            if let Some(&val) = r {
                result = Some(val);
            }
        }).unwrap();
        if let Some(val) = result {
            self.cbuf.push(val);
            Some(&self.cbuf[0])
        } else {
            None
        }
    }

    fn cache_set(&mut self, k: (u32, u32, u32), v: u32) {
        let cache_entry = CacheEntry {
            fname: self.fname.clone(),
            group: vec![k.0],
            other_args: vec![k.1, k.2],
        };
        let db = DATABASE.lock().unwrap();
        db.write(|db| {
            db.insert(cache_entry, v);
        }).expect("Error writing to cache database file!");
        db.save().expect("Error saving to cache database file!");
    }

    fn cache_remove(&mut self, _k: &(u32, u32, u32)) -> Option<u32> {
        unimplemented!()
    }

    fn cache_clear(&mut self) {
        unimplemented!()
    }

    fn cache_reset(&mut self) {
        unimplemented!()
    }

    fn cache_size(&self) -> usize {
        unimplemented!()
    }
}
