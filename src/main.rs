// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2025 Benjamin Grolleau and Angelo Tunney

use arbor::{
    cli::args,
    config::OutputFormat,
    fs_scan::walk,
    git::{collect_git_states, enrich_with_git, write_git_branch},
    helpers, logger,
    renderer::{count, json, stdout},
};
use clap::Parser;
use log::{debug, error};
use std::{path::PathBuf, time::Instant};

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

    let t_start = Instant::now();

    match walk::walk_path(&current_dir, &config.walk) {
        Ok(mut node) => {
            let mut out = std::io::stdout().lock();
            let res = match config.output {
                OutputFormat::Count => count::render(&mut out, &node),

                OutputFormat::Json | OutputFormat::Tree => {
                    if config.git.enabled {
                        let git_states = collect_git_states(&current_dir);
                        let mut buf = String::new();
                        enrich_with_git(&mut node, &git_states, &mut buf);
                    }
                    if config.git.show_branch {
                        write_git_branch(&mut out, &current_dir);
                    }
                    match config.output {
                        OutputFormat::Json => json::render(&mut out, &node),
                        OutputFormat::Tree => stdout::render(&mut out, &node, &config.render),
                        _ => unreachable!(),
                    }
                }
            };
            if let Err(e) = res {
                error!("write error: {e}");
                std::process::exit(1);
            }
        }
        Err(e) => {
            error!("❌ - failed to execute STree on this directory! {e}");
            std::process::exit(1);
        }
    }

    if config.runtime.measure_time {
        eprintln!("time: {}", helpers::format_duration(t_start.elapsed()));
    }
}
