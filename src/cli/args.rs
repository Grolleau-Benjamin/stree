// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2025 Benjamin Grolleau and Angelo Tunney

use crate::config;
use crate::version;
use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq)]
pub enum ColorMode {
    Auto,
    Always,
    Never,
}

#[derive(Parser, Debug)]
#[command(
    name = "arbor",
    version = version::SHORT,
    long_version = version::LONG,
    about = " Arbor — a modern and smart reimplementation of the classic `tree` command.",
    long_about = r#"
Arbor enhances the classic `tree` by adding colorized output, .gitignore
integration, file-type icons, and Git status indicators. It allows you to
navigate your projects more clearly and efficiently."#,
    author = "Benjamin Grolleau <benjamin.grolleau@outlook.com>, Angelo Tunney <angelo.tny@hotmail.com>",
    help_template = r#"{before-help}{author}, {version}
{about-with-newline}
USAGE:
  {usage}

OPTIONS:
{options}
{after-help}"#
)]
pub struct Args {
    // ------------
    // BASIC OPTIONS
    // -----------
    /// Show files that are listed in .gitignore (ignored by default)
    #[arg(long, short = 'G')]
    pub show_gitignored: bool,

    /// Inlcude hidden files and directories (starting with .)
    #[arg(long, short = 'H')]
    pub show_hiddens: bool,

    /// Colorize the output (Auto, Always, Never).
    #[arg(long, short = 'c', value_enum, default_value_t = ColorMode::Auto)]
    pub color: ColorMode,

    /// Add icons for known file types and directories
    #[arg(long, short = 'i')]
    pub icons: bool,

    // ------------
    // TREE RENDERING
    // ------------
    /// Limit the displayed depth of the tree
    #[arg(long, short = 'd', value_name = "N")]
    pub depth: Option<usize>,

    // -------------------
    // GIT INTEGRATION
    // -------------------
    /// Display Git status indicators (modified, staged, untracked, etc.)
    #[arg(long, short = 'g')]
    pub git: bool,

    /// Show the current branch name next to the root
    #[arg(long, short = 'b')]
    pub git_branch: bool,

    // -------------------
    // OUTPUT CONTROL
    // -------------------
    /// Output the tree as a JSON structure
    #[arg(long, short = 'j')]
    pub json: bool,

    /// Print only the number of files and directories
    #[arg(long, short = 'n')]
    pub count: bool,

    /// Measure and display execution time
    #[arg(long, short = 't')]
    pub time: bool,

    // -------------------
    // UTILITY
    // -------------------
    /// Enable detailed logging
    #[arg(long, short = 'v')]
    pub verbose: bool,

    /// Root directory to explore (default: current directory)
    #[arg(default_value = ".")]
    pub root: String,
}

impl Args {
    pub fn build_config(self) -> Result<config::AppConfig, String> {
        config::AppConfig::from_raw(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn defaults_are_correct() {
        let args = Args::try_parse_from(["arbor"]).unwrap();
        assert_eq!(args.root, ".");
        assert!(!args.show_gitignored);
        assert!(!args.show_hiddens);
        assert_eq!(args.color, ColorMode::Auto);
        assert!(!args.icons);
        assert!(args.depth.is_none());
        assert!(!args.git);
        assert!(!args.git_branch);
        assert!(!args.json);
        assert!(!args.count);
        assert!(!args.time);
        assert!(!args.verbose);
    }

    #[test]
    fn root_positional_is_parsed() {
        let args = Args::try_parse_from(["arbor", "/home/vatara"]).unwrap();
        assert_eq!(args.root, "/home/vatara");
    }

    #[test]
    fn gitignore_flag() {
        let args = Args::try_parse_from(["arbor", "--show-gitignored"]).unwrap();
        assert!(args.show_gitignored);
    }

    #[test]
    fn show_hiddens_flag() {
        let args = Args::try_parse_from(["arbor", "--show-hiddens"]).unwrap();
        assert!(args.show_hiddens);
    }

    #[test]
    fn color_variants_parse() {
        let a = Args::try_parse_from(["arbor", "--color", "auto"]).unwrap();
        assert_eq!(a.color, ColorMode::Auto);

        let b = Args::try_parse_from(["arbor", "--color", "always"]).unwrap();
        assert_eq!(b.color, ColorMode::Always);

        let c = Args::try_parse_from(["arbor", "--color", "never"]).unwrap();
        assert_eq!(c.color, ColorMode::Never);
    }

    #[test]
    fn icons_flag() {
        let args = Args::try_parse_from(["arbor", "--icons"]).unwrap();
        assert!(args.icons);
    }

    #[test]
    fn depth_value_is_parsed() {
        let args = Args::try_parse_from(["arbor", "--depth", "3"]).unwrap();
        assert_eq!(args.depth, Some(3));
    }

    #[test]
    fn git_flag() {
        let args = Args::try_parse_from(["arbor", "--git"]).unwrap();
        assert!(args.git);
    }

    #[test]
    fn git_branch_flag() {
        let args = Args::try_parse_from(["arbor", "--git-branch"]).unwrap();
        assert!(args.git_branch);
    }

    #[test]
    fn json_flag() {
        let args = Args::try_parse_from(["arbor", "--json"]).unwrap();
        assert!(args.json);
    }

    #[test]
    fn count_flag() {
        let args = Args::try_parse_from(["arbor", "--count"]).unwrap();
        assert!(args.count);
    }

    #[test]
    fn time_flag() {
        let args = Args::try_parse_from(["arbor", "--time"]).unwrap();
        assert!(args.time);
    }

    #[test]
    fn verbose_flag() {
        let args = Args::try_parse_from(["arbor", "--verbose"]).unwrap();
        assert!(args.verbose);
    }

    #[test]
    fn full_combination_parses() {
        let args = Args::try_parse_from([
            "arbor",
            "--show-gitignored",
            "--show-hiddens",
            "--color",
            "always",
            "--icons",
            "--depth",
            "2",
            "--git",
            "--git-branch",
            "--time",
            "--verbose",
            "root_dir",
        ])
        .unwrap();

        assert!(args.show_gitignored);
        assert!(args.show_hiddens);
        assert_eq!(args.color, ColorMode::Always);
        assert!(args.icons);
        assert_eq!(args.depth, Some(2));
        assert!(args.git);
        assert!(args.git_branch);
        assert!(args.time);
        assert!(args.verbose);
        assert_eq!(args.root, "root_dir");
    }

    #[test]
    fn rejects_invalid_enum_value() {
        let err = Args::try_parse_from(["arbor", "--color", "rainbow"]).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("rainbow"));
        assert!(msg.contains("possible values"));
    }
}
