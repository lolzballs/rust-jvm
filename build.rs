extern crate glob;

use std::process::Command;
use glob::glob;

fn main() {
    println!("cargo:rerun-if-changed=runtime");
    for entry in glob("runtime/**/*.java").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                Command::new("javac")
                    .current_dir("runtime")
                    .arg(path.strip_prefix("runtime").expect("failed to strip runtime"))
                    .status()
                    .expect("failed to execute javac")
                    .success();
                println!("cargo:rerun-if-changed={:?}", path);
            }
            Err(e) => {
                println!("cargo:warning={:?}", e);
            }
        }
    }
}
