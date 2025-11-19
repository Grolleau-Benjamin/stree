// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2025 Benjamin Grolleau and Angelo Tunney

use std::path::PathBuf;

use crate::cli::args::{Args, ColorMode};

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Tree,
    Json,
    Count,
}

#[derive(Debug, Clone)]
pub struct WalkOptions {
    pub follow_gitignore: bool,
    pub include_hidden: bool,
    pub depth: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct RenderOptions {
    pub color: ColorMode,
    pub icons: bool,
    pub git: bool,
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
    pub output: OutputFormat,
    pub runtime: RuntimeOptions,
}

impl AppConfig {
    pub fn from_raw(raw: Args) -> Result<Self, String> {
        if raw.json && raw.count {
            return Err(String::from("--json and --count are mutually exclusive"));
        }
        if let Some(d) = raw.depth
            && d == 0
        {
            return Err(String::from("--depth must be >= 1 when provided"));
        }

        let output = if raw.json {
            OutputFormat::Json
        } else if raw.count {
            OutputFormat::Count
        } else {
            OutputFormat::Tree
        };

        Ok(Self {
            walk: WalkOptions {
                follow_gitignore: !raw.show_gitignored,
                include_hidden: raw.show_hiddens,
                depth: raw.depth,
            },
            render: RenderOptions {
                color: raw.color,
                icons: raw.icons,
                git: raw.git,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::args::{Args, ColorMode};
    use clap::Parser;

    #[test]
    fn conflict_json_and_count() {
        let args = Args::try_parse_from(["arbor", "--json", "--count"]).unwrap();
        let err = AppConfig::from_raw(args).unwrap_err();
        assert!(err.contains("mutually exclusive"));
    }

    #[test]
    fn depth_zero_is_rejected() {
        let args = Args::try_parse_from(["arbor", "--depth", "0"]).unwrap();
        let err = AppConfig::from_raw(args).unwrap_err();
        assert!(err.contains("--depth must be >= 1"));
    }

    #[test]
    fn defaults_map_to_stdout_mode() {
        let args = Args::try_parse_from(["arbor"]).unwrap();
        let cfg = AppConfig::from_raw(args).unwrap();

        assert!(matches!(cfg.output, OutputFormat::Tree));

        assert!(cfg.walk.follow_gitignore);
        assert!(!cfg.walk.include_hidden);
        assert!(cfg.walk.depth.is_none());

        assert_eq!(cfg.render.color, ColorMode::Auto);
        assert!(!cfg.render.icons);

        assert!(!cfg.git.enabled);
        assert!(!cfg.git.show_branch);

        assert!(!cfg.runtime.measure_time);
        assert!(!cfg.runtime.verbose);
        assert_eq!(cfg.runtime.root, PathBuf::from("."));
    }

    #[test]
    fn maps_flags_to_config_correctly() {
        let args = Args::try_parse_from([
            "arbor",
            // Walk
            "--show-gitignored",
            "--show-hiddens",
            "--depth",
            "3",
            // Render
            "--color",
            "never",
            "--icons",
            // Git
            "--git",
            "--git-branch",
            // Runtime
            "--time",
            "--verbose",
            // Root
            "root_dir",
        ])
        .unwrap();

        let cfg = AppConfig::from_raw(args).unwrap();

        assert!(!cfg.walk.follow_gitignore);
        assert!(cfg.walk.include_hidden);
        assert_eq!(cfg.walk.depth, Some(3));

        assert_eq!(cfg.render.color, ColorMode::Never);
        assert!(cfg.render.icons);

        assert!(cfg.git.enabled);
        assert!(cfg.git.show_branch);

        assert!(matches!(cfg.output, OutputFormat::Tree));

        assert!(cfg.runtime.measure_time);
        assert!(cfg.runtime.verbose);
        assert_eq!(cfg.runtime.root, PathBuf::from("root_dir"));
    }

    #[test]
    fn selects_output_modes_json_and_count() {
        let args_json = Args::try_parse_from(["arbor", "--json"]).unwrap();
        let cfg_json = AppConfig::from_raw(args_json).unwrap();
        assert!(matches!(cfg_json.output, OutputFormat::Json));

        let args_count = Args::try_parse_from(["arbor", "--count"]).unwrap();
        let cfg_count = AppConfig::from_raw(args_count).unwrap();
        assert!(matches!(cfg_count.output, OutputFormat::Count));
    }
}
