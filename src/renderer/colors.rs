// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2025 Benjamin Grolleau and Angelo Tunney

use crate::model::node::GitState;
use phf::phf_map;

pub static EXT_COLORS: phf::Map<&'static str, &'static [u8]> = phf_map! {
    "rs"   => b"\x1b[38;5;216m",
    "c"    => b"\x1b[38;5;110m",
    "h"    => b"\x1b[38;5;110m",
    "cpp"  => b"\x1b[38;5;109m",
    "hpp"  => b"\x1b[38;5;109m",
    "cc"   => b"\x1b[38;5;109m",
    "go"   => b"\x1b[38;5;115m",
    "py"   => b"\x1b[38;5;186m",
    "js"   => b"\x1b[38;5;187m",
    "jsx"  => b"\x1b[38;5;110m",
    "ts"   => b"\x1b[38;5;110m",
    "tsx"  => b"\x1b[38;5;111m",
    "json" => b"\x1b[38;5;180m",
    "toml" => b"\x1b[38;5;144m",
    "yaml" => b"\x1b[38;5;187m",
    "yml"  => b"\x1b[38;5;187m",
    "ini"  => b"\x1b[38;5;144m",
    "md"   => b"\x1b[38;5;182m",
    "mdx"  => b"\x1b[38;5;182m",
    "txt"  => b"\x1b[38;5;250m",
    "png"  => b"\x1b[38;5;183m",
    "jpg"  => b"\x1b[38;5;183m",
    "jpeg" => b"\x1b[38;5;183m",
    "gif"  => b"\x1b[38;5;183m",
    "svg"  => b"\x1b[38;5;183m",
    "pdf"  => b"\x1b[38;5;174m",
};

const DIR_COLOR: &[u8] = b"\x1b[38;5;110m";
const FILE_DEFAULT: &[u8] = b"\x1b[38;5;252m";
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
    (b"", b""),                                      // Clean
    (b"\x1b[33m", b" \x1b[1m~\x1b[0m"),              // Modified
    (b"\x1b[32m", b" \x1b[1m+\x1b[0m"),              // Staged
    (b"\x1b[31m", b" \x1b[1m?\x1b[0m"),              // Untracked
    (b"\x1b[38;5;244m", b" \x1b[1m(i)Hello\x1b[0m"), // Ignored
    (b"\x1b[36m", b" \x1b[1m\xE2\x86\x92\x1b[0m"),   // Renamed →
    (b"\x1b[31m", b" \x1b[1m\xE2\x9C\x96\x1b[0m"),   // Deleted ✖
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
