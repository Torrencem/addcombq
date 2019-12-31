
use std::env::var;

use std::path::PathBuf;

use std::fs;
use std::io;

fn main() {
    match var("HOME") {
        Ok(home) => {
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
            let acpath = format!("{:?}", acpath).to_string();
            println!("cargo:rustc-env=AC_DATABASE={}", &acpath[1..acpath.len()-1]);
        },
        Err(_) => {
            
        }
    }
}
