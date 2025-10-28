use clap::Parser;
use log::{debug, error};
use std::path::PathBuf;
use stree::{cli::args, fs_scan::walk, logger, renderer::stdout};

fn main() {
    let raw = args::Args::parse();

    logger::init_logger(raw.verbose);
    let config = match raw.build_config() {
        Ok(cfg) => cfg,
        Err(msg) => {
            eprintln!("Error: {msg}");
            std::process::exit(1);
        }
    };

    debug!("Config loaded successfully: {:?}", config);

    let current_dir: PathBuf = config.runtime.root;
    debug!("Running STree in: {}", current_dir.display());

    if let Ok(node) = walk::walk_path(&current_dir) {
        stdout::render(&node);
    } else {
        error!("❌ - failed to execute STree on this directory!");
    }
}
