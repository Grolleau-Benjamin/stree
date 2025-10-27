use clap::Parser;
use std::path::PathBuf;
use stree::{cli::args, fs_scan::walk, renderer::stdout};

fn main() {
    let raw = args::Args::parse();

    let config = match stree::config::AppConfig::from_raw(raw) {
        Ok(cfg) => cfg,
        Err(msg) => {
            eprintln!("Error: {msg}");
            std::process::exit(1);
        }
    };

    println!("Config loaded successfully: {:?}", config);

    let current_dir: PathBuf = config.runtime.root;
    println!("Running STree in: {}", current_dir.display());

    if let Ok(node) = walk::walk_path(&current_dir) {
        stdout::render(&node);
    } else {
        println!("❌ - failed to execute STree on this directory!");
    }
}
