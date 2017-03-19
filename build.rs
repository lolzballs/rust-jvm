extern crate glob;

use std::env;
use std::fs;
use std::path;
use std::process::Command;
use glob::glob;

fn main() {
    // The output directory is at OUT_DIR/runtime
    let out = {
        let mut out = path::PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR was not defined"));
        out.push("runtime");
        out
    };
    if !out.is_dir() {
        fs::create_dir(&out).expect("failed to create runtime output directory");
    }

    println!("cargo:rerun-if-changed=runtime");
    for entry in glob("runtime/**/*.java").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                Command::new("javac")
                    .current_dir("runtime")
                    .arg("-d")
                    .arg(&out)
                    .arg(path.strip_prefix("runtime").expect("failed to strip runtime"))
                    .status()
                    .expect("failed to execute javac")
                    .success();
                println!("cargo:rerun-if-changed={}", path.display());
            }
            Err(e) => {
                println!("cargo:warning={:?}", e);
            }
        }
    }
}
