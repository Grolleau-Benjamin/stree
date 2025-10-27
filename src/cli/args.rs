use crate::config;
use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum ColorMode {
    AUTO,
    ALWAYS,
    NEVER,
}

#[derive(Parser, Debug)]
#[command(
    name = "stree",
    version,
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
