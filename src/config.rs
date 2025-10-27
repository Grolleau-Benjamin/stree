use std::path::PathBuf;

use crate::cli::args::{Args, ColorMode};

#[derive(Debug, Clone, Copy)]
pub enum OutputMode {
    STDOUT,
    JSON,
    COUNT,
}

#[derive(Debug, Clone)]
pub struct WalkOptions {
    pub follow_gitignore: bool,
    pub include_hidden: bool,
    pub depth: Option<usize>,
    pub dirs_only: bool,
    pub files_only: bool,
    pub prune_empty: bool,
}

#[derive(Debug, Clone)]
pub struct RenderOptions {
    pub color: ColorMode,
    pub icons: bool,
}

#[derive(Debug, Clone)]
pub struct GitOptions {
    pub enabled: bool,
    pub show_branch: bool,
}

#[derive(Debug, Clone)]
pub struct RuntimeOptions {
    pub measure_time: bool,
    pub verbose: bool,
    pub root: PathBuf,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub walk: WalkOptions,
    pub render: RenderOptions,
    pub git: GitOptions,
    pub output: OutputMode,
    pub runtime: RuntimeOptions,
}

impl AppConfig {
    pub fn from_raw(raw: Args) -> Result<Self, String> {
        if raw.dirs_only && raw.files_only {
            return Err(String::from(
                "--dirs-only and --files-only are mutually exclusive",
            ));
        }
        if raw.json && raw.count {
            return Err(String::from("--json and --count are mutually exclusive"));
        }
        if let Some(d) = raw.depth
            && d == 0
        {
            return Err(String::from("--depth must be >= 1 when provided"));
        }

        let output = if raw.json {
            OutputMode::JSON
        } else if raw.count {
            OutputMode::COUNT
        } else {
            OutputMode::STDOUT
        };

        Ok(Self {
            walk: WalkOptions {
                follow_gitignore: raw.gitignore,
                include_hidden: raw.hidden_files,
                depth: raw.depth,
                dirs_only: raw.dirs_only,
                files_only: raw.files_only,
                prune_empty: raw.prune_empty,
            },
            render: RenderOptions {
                color: raw.color,
                icons: raw.icons,
            },
            git: GitOptions {
                enabled: raw.git,
                show_branch: raw.git_branch,
            },
            output,
            runtime: RuntimeOptions {
                measure_time: raw.time,
                verbose: raw.verbose,
                root: raw.root.into(),
            },
        })
    }
}
