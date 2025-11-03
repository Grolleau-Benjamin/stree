// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2025 Benjamin Grolleau and Angelo Tunney

use crate::model::node::GitState;
use phf::phf_map;

pub static EXT_COLORS: phf::Map<&'static str, &'static [u8]> = phf_map! {
    "rs" => b"\x1b[1;32m",
    "c" => b"\x1b[36m",
    "h" => b"\x1b[36m",
    "cpp" => b"\x1b[36m",
    "hpp" => b"\x1b[36m",
    "cc" => b"\x1b[36m",
    "go" => b"\x1b[96m",
    "py" => b"\x1b[33m",
    "ts" => b"\x1b[94m",
    "tsx" => b"\x1b[94m",
    "js" => b"\x1b[34m",
    "jsx" => b"\x1b[34m",
    "json" => b"\x1b[93m",
    "toml" => b"\x1b[93m",
    "yaml" => b"\x1b[93m",
    "yml" => b"\x1b[93m",
    "md" => b"\x1b[1;95m",
    "mdx" => b"\x1b[1;95m",
    "txt" => b"\x1b[38;5;244m",
    "png" => b"\x1b[93m",
    "jpg" => b"\x1b[93m",
    "jpeg" => b"\x1b[93m",
    "gif" => b"\x1b[93m",
    "svg" => b"\x1b[93m",
    "pdf" => b"\x1b[31m",
};

const DIR_COLOR: &[u8] = b"\x1b[1;34m";
const FILE_DEFAULT: &[u8] = b"\x1b[37m";
pub const RESET: &[u8] = b"\x1b[0m";

#[inline]
pub fn color_for_name(name: &str, is_dir: bool) -> &'static [u8] {
    if is_dir {
        return DIR_COLOR;
    }
    match name.rsplit('.').next() {
        Some(ext) => EXT_COLORS.get(ext).copied().unwrap_or(FILE_DEFAULT),
        None => FILE_DEFAULT,
    }
}

const GIT_MARKERS: [(&[u8], &[u8]); 7] = [
    (b"", b""),                      // Clean
    (b"\x1b[33m", b" ~"),            // Modified
    (b"\x1b[32m", b" +"),            // Staged
    (b"\x1b[31m", b" ?"),            // Untracked
    (b"\x1b[38;5;244m", b" (i)"),    // Ignored
    (b"\x1b[36m", b" \xE2\x86\x92"), // Renamed →
    (b"\x1b[31m", b" \xE2\x9C\x96"), // Deleted ✖
];

#[inline]
fn git_idx(g: GitState) -> usize {
    match g {
        GitState::Clean => 0,
        GitState::Modified => 1,
        GitState::Staged => 2,
        GitState::Untracked => 3,
        GitState::Ignored => 4,
        GitState::Renamed => 5,
        GitState::Deleted => 6,
    }
}

#[inline]
pub fn git_marker(g: GitState) -> Option<(&'static [u8], &'static [u8])> {
    let (c, s) = GIT_MARKERS[git_idx(g)];
    if s.is_empty() { None } else { Some((c, s)) }
}
