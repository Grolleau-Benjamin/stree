use crate::config;
use crate::version;
use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq)]
pub enum ColorMode {
    AUTO,
    ALWAYS,
    NEVER,
}

#[derive(Parser, Debug)]
#[command(
    name = "stree",
    version = version::SHORT,
    long_version = version::LONG,
    about = " Stree — a modern and smart reimplementation of the classic `tree` command.",
    long_about = r#"
STree enhances the classic `tree` by adding colorized output, .gitignore
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
    #[arg(long)]
    pub gitignore: bool,

    /// Inlcude hidden files and directories (starting with .)
    #[arg(long)]
    pub hidden_files: bool,

    /// Colorize the output (Auto, Always, Never).
    #[arg(long, value_enum, default_value_t = ColorMode::AUTO)]
    pub color: ColorMode,

    /// Add icons for known file types and directories
    #[arg(long)]
    pub icons: bool,

    // ------------
    // TREE RENDERING
    // ------------
    /// Limit the displayed depth of the tree
    #[arg(long, value_name = "N")]
    pub depth: Option<usize>,

    /// Display only directories
    #[arg(long)]
    pub dirs_only: bool,

    /// Display only files
    #[arg(long)]
    pub files_only: bool,

    // -------------------
    // FILTERING
    // -------------------
    /// Hide empty directories
    #[arg(long)]
    pub prune_empty: bool,

    // -------------------
    // GIT INTEGRATION
    // -------------------
    /// Display Git status indicators (modified, staged, untracked, etc.)
    #[arg(long)]
    pub git: bool,

    /// Show the current branch name next to the root
    #[arg(long)]
    pub git_branch: bool,

    // -------------------
    // OUTPUT CONTROL
    // -------------------
    /// Output the tree as a JSON structure
    #[arg(long)]
    pub json: bool,

    /// Print only the number of files and directories
    #[arg(long)]
    pub count: bool,

    /// Measure and display execution time
    #[arg(long)]
    pub time: bool,

    // -------------------
    // UTILITY
    // -------------------
    /// Enable detailed logging
    #[arg(long)]
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
        let args = Args::try_parse_from(["stree"]).unwrap();
        assert_eq!(args.root, ".");
        assert!(!args.gitignore);
        assert!(!args.hidden_files);
        assert_eq!(args.color, ColorMode::AUTO);
        assert!(!args.icons);
        assert!(args.depth.is_none());
        assert!(!args.dirs_only);
        assert!(!args.files_only);
        assert!(!args.prune_empty);
        assert!(!args.git);
        assert!(!args.git_branch);
        assert!(!args.json);
        assert!(!args.count);
        assert!(!args.time);
        assert!(!args.verbose);
    }

    #[test]
    fn root_positional_is_parsed() {
        let args = Args::try_parse_from(["stree", "/home/vatara"]).unwrap();
        assert_eq!(args.root, "/home/vatara");
    }

    #[test]
    fn gitignore_flag() {
        let args = Args::try_parse_from(["stree", "--gitignore"]).unwrap();
        assert!(args.gitignore);
    }

    #[test]
    fn hidden_files_flag() {
        let args = Args::try_parse_from(["stree", "--hidden-files"]).unwrap();
        assert!(args.hidden_files);
    }

    #[test]
    fn color_variants_parse() {
        let a = Args::try_parse_from(["stree", "--color", "auto"]).unwrap();
        assert_eq!(a.color, ColorMode::AUTO);

        let b = Args::try_parse_from(["stree", "--color", "always"]).unwrap();
        assert_eq!(b.color, ColorMode::ALWAYS);

        let c = Args::try_parse_from(["stree", "--color", "never"]).unwrap();
        assert_eq!(c.color, ColorMode::NEVER);
    }

    #[test]
    fn icons_flag() {
        let args = Args::try_parse_from(["stree", "--icons"]).unwrap();
        assert!(args.icons);
    }

    #[test]
    fn depth_value_is_parsed() {
        let args = Args::try_parse_from(["stree", "--depth", "3"]).unwrap();
        assert_eq!(args.depth, Some(3));
    }

    #[test]
    fn dirs_only_flag() {
        let args = Args::try_parse_from(["stree", "--dirs-only"]).unwrap();
        assert!(args.dirs_only);
        assert!(!args.files_only);
    }

    #[test]
    fn files_only_flag() {
        let args = Args::try_parse_from(["stree", "--files-only"]).unwrap();
        assert!(args.files_only);
        assert!(!args.dirs_only);
    }

    #[test]
    fn prune_empty_flag() {
        let args = Args::try_parse_from(["stree", "--prune-empty"]).unwrap();
        assert!(args.prune_empty);
    }

    #[test]
    fn git_flag() {
        let args = Args::try_parse_from(["stree", "--git"]).unwrap();
        assert!(args.git);
    }

    #[test]
    fn git_branch_flag() {
        let args = Args::try_parse_from(["stree", "--git-branch"]).unwrap();
        assert!(args.git_branch);
    }

    #[test]
    fn json_flag() {
        let args = Args::try_parse_from(["stree", "--json"]).unwrap();
        assert!(args.json);
    }

    #[test]
    fn count_flag() {
        let args = Args::try_parse_from(["stree", "--count"]).unwrap();
        assert!(args.count);
    }

    #[test]
    fn time_flag() {
        let args = Args::try_parse_from(["stree", "--time"]).unwrap();
        assert!(args.time);
    }

    #[test]
    fn verbose_flag() {
        let args = Args::try_parse_from(["stree", "--verbose"]).unwrap();
        assert!(args.verbose);
    }

    #[test]
    fn full_combination_parses() {
        let args = Args::try_parse_from([
            "stree",
            "--gitignore",
            "--hidden-files",
            "--color",
            "always",
            "--icons",
            "--depth",
            "2",
            "--dirs-only",
            "--prune-empty",
            "--git",
            "--git-branch",
            "--time",
            "--verbose",
            "root_dir",
        ])
        .unwrap();

        assert!(args.gitignore);
        assert!(args.hidden_files);
        assert_eq!(args.color, ColorMode::ALWAYS);
        assert!(args.icons);
        assert_eq!(args.depth, Some(2));
        assert!(args.dirs_only);
        assert!(args.prune_empty);
        assert!(args.git);
        assert!(args.git_branch);
        assert!(args.time);
        assert!(args.verbose);
        assert_eq!(args.root, "root_dir");
    }

    #[test]
    fn rejects_invalid_enum_value() {
        let err = Args::try_parse_from(["stree", "--color", "rainbow"]).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("rainbow"));
        assert!(msg.contains("possible values"));
    }
}
