use std::{env, path::PathBuf};
use stree::{fs_scan::walk, renderer::stdout};

fn main() {
    let current_dir: PathBuf = env::current_dir().expect("Failed to get current directory");
    println!("Running STree in: {}", current_dir.display());

    if let Ok(node) = walk::walk_path(&current_dir) {
        stdout::render(&node);
    } else {
        println!("❌ - failed to execute STree on this directory.");
    }
}
