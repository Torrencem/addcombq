// Build script: run the python script to build documentation
// (requires that python3 is installed on computer)

use std::process::Command;

fn main() {
    Command::new("python3").args(&["build.py"])
        .current_dir("doc/")
        .status().expect("failed to build documentation! doc/build.py failed");
}
