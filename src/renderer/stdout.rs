// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2025 Benjamin Grolleau and Angelo Tunney

use crate::{
    cli::args::ColorMode,
    config::RenderOptions,
    model::node::Node,
    renderer::{colors, icons},
};
use std::io::{self, Write};

type NameFn<W> = fn(&mut W, &Node) -> io::Result<()>;

pub fn render<W: Write>(mut w: W, root: &Node, opts: &RenderOptions) -> io::Result<()> {
    let write_name: NameFn<W> = match (opts.icons, opts.color) {
        (false, ColorMode::Never) => write_plain,
        (false, ColorMode::Auto) => write_plain_gitonly,
        (false, ColorMode::Always) => write_plain_full,
        (true, ColorMode::Never) => write_icon_plain,
        (true, ColorMode::Auto) => write_icon_gitonly,
        (true, ColorMode::Always) => write_icon_full,
    };
    write_name(&mut w, root)?;
    w.write_all(b"\n")?;
    let children = root.children_slice();
    let last_idx = children.len().saturating_sub(1);
    for (i, child) in children.iter().enumerate() {
        render_node(&mut w, child, "", i == last_idx, write_name)?;
    }
    Ok(())
}

fn render_node<W: Write>(
    w: &mut W,
    node: &Node,
    prefix: &str,
    is_last: bool,
    write_name: NameFn<W>,
) -> io::Result<()> {
    let branch = if is_last { "└── " } else { "├── " };
    w.write_all(prefix.as_bytes())?;
    w.write_all(branch.as_bytes())?;
    write_name(w, node)?;
    w.write_all(b"\n")?;
    let mut new_prefix = String::with_capacity(prefix.len() + 4);
    new_prefix.push_str(prefix);
    new_prefix.push_str(if is_last { "    " } else { "│   " });
    let children = node.children_slice();
    let last_idx = children.len().saturating_sub(1);
    for (i, child) in children.iter().enumerate() {
        render_node(w, child, &new_prefix, i == last_idx, write_name)?;
    }
    Ok(())
}

#[inline]
fn write_plain<W: Write>(w: &mut W, n: &Node) -> io::Result<()> {
    w.write_all(n.name.as_bytes())?;
    if n.is_dir() {
        w.write_all(b"/")?;
    }
    Ok(())
}

#[inline]
fn write_plain_gitonly<W: Write>(w: &mut W, n: &Node) -> io::Result<()> {
    w.write_all(n.name.as_bytes())?;
    if n.is_dir() {
        w.write_all(b"/")?;
    }
    if let Some((c, s)) = n.meta.git.and_then(colors::git_marker) {
        w.write_all(c)?;
        w.write_all(s)?;
        w.write_all(colors::RESET)?;
    }
    Ok(())
}

#[inline]
fn write_plain_full<W: Write>(w: &mut W, n: &Node) -> io::Result<()> {
    w.write_all(colors::color_for_name(&n.name, n.is_dir()))?;
    w.write_all(n.name.as_bytes())?;
    if n.is_dir() {
        w.write_all(b"/")?;
    }
    w.write_all(colors::RESET)?;
    if let Some((c, s)) = n.meta.git.and_then(colors::git_marker) {
        w.write_all(c)?;
        w.write_all(s)?;
        w.write_all(colors::RESET)?;
    }
    Ok(())
}

#[inline]
fn write_icon_plain<W: Write>(w: &mut W, n: &Node) -> io::Result<()> {
    let icon = if n.is_dir() {
        icons::dir_icon(&n.name)
    } else {
        icons::file_icon(&n.name)
    };
    let mut buf = [0u8; 4];
    w.write_all(icon.encode_utf8(&mut buf).as_bytes())?;
    w.write_all(b" ")?;
    w.write_all(n.name.as_bytes())?;
    if n.is_dir() {
        w.write_all(b"/")?;
    }
    Ok(())
}

#[inline]
fn write_icon_gitonly<W: Write>(w: &mut W, n: &Node) -> io::Result<()> {
    let icon = if n.is_dir() {
        icons::dir_icon(&n.name)
    } else {
        icons::file_icon(&n.name)
    };
    let mut buf = [0u8; 4];
    w.write_all(icon.encode_utf8(&mut buf).as_bytes())?;
    w.write_all(b" ")?;
    w.write_all(n.name.as_bytes())?;
    if n.is_dir() {
        w.write_all(b"/")?;
    }
    if let Some((c, s)) = n.meta.git.and_then(colors::git_marker) {
        w.write_all(c)?;
        w.write_all(s)?;
        w.write_all(colors::RESET)?;
    }
    Ok(())
}

#[inline]
fn write_icon_full<W: Write>(w: &mut W, n: &Node) -> io::Result<()> {
    let icon = if n.is_dir() {
        icons::dir_icon(&n.name)
    } else {
        icons::file_icon(&n.name)
    };
    let mut buf = [0u8; 4];
    w.write_all(icon.encode_utf8(&mut buf).as_bytes())?;
    w.write_all(b" ")?;
    w.write_all(colors::color_for_name(&n.name, n.is_dir()))?;
    w.write_all(n.name.as_bytes())?;
    if n.is_dir() {
        w.write_all(b"/")?;
    }
    w.write_all(colors::RESET)?;
    if let Some((c, s)) = n.meta.git.and_then(colors::git_marker) {
        w.write_all(c)?;
        w.write_all(s)?;
        w.write_all(colors::RESET)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::args::ColorMode;
    use crate::config::RenderOptions;
    use crate::model::node::{GitState, Node};

    fn opts(icons: bool, color: ColorMode) -> RenderOptions {
        RenderOptions { icons, color }
    }

    #[test]
    fn classic_single_dir() {
        let root = Node::new_dir("root", vec![]);
        let mut buf = Vec::new();
        let o = opts(false, ColorMode::Never);
        render(&mut buf, &root, &o).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "root/\n");
    }

    #[test]
    fn classic_tree() {
        let root = Node::new_dir(
            "root",
            vec![
                Node::new_dir(
                    "src",
                    vec![Node::new_file("main.rs", 1), Node::new_file("lib.rs", 1)],
                ),
                Node::new_file("README.md", 1),
            ],
        );
        let mut buf = Vec::new();
        let o = opts(false, ColorMode::Never);
        render(&mut buf, &root, &o).unwrap();
        let got = String::from_utf8(buf).unwrap();
        let expected = "\
root/
├── src/
│   ├── main.rs
│   └── lib.rs
└── README.md
";
        assert_eq!(got, expected);
    }

    #[test]
    fn icons_single_dir() {
        let root = Node::new_dir("root", vec![]);
        let mut buf = Vec::new();
        let o = opts(true, ColorMode::Never);
        render(&mut buf, &root, &o).unwrap();
        let got = String::from_utf8(buf).unwrap();
        let expected = format!("{} root/\n", '\u{f115}');
        assert_eq!(got, expected);
    }

    #[test]
    fn icons_tree() {
        let root = Node::new_dir(
            "root",
            vec![
                Node::new_dir(
                    "src",
                    vec![Node::new_file("main.rs", 1), Node::new_file("lib.rs", 1)],
                ),
                Node::new_file("README.md", 1),
            ],
        );
        let mut buf = Vec::new();
        let o = opts(true, ColorMode::Never);
        render(&mut buf, &root, &o).unwrap();
        let got = String::from_utf8(buf).unwrap();
        let expected = "\
\u{f115} root/
├── \u{f08de} src/
│   ├── \u{e7a8} main.rs
│   └── \u{e7a8} lib.rs
└── \u{f00ba} README.md
";
        assert_eq!(got, expected);
    }

    #[test]
    fn auto_git_only_colors_when_git_present() {
        let mut f = Node::new_file("a.txt", 1);
        f.meta.git = Some(GitState::Modified);
        let mut buf = Vec::new();
        let o = opts(false, ColorMode::Auto);
        render(&mut buf, &f, &o).unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert!(s.starts_with("a.txt"));
        assert!(s.contains('\u{1b}'));
    }

    #[test]
    fn auto_no_git_means_no_color() {
        let f = Node::new_file("a.txt", 1);
        let mut buf = Vec::new();
        let o = opts(false, ColorMode::Auto);
        render(&mut buf, &f, &o).unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert_eq!(s, "a.txt\n");
        assert!(!s.contains('\u{1b}'));
    }

    #[test]
    fn always_colors_names() {
        let f = Node::new_file("a.txt", 1);
        let mut buf = Vec::new();
        let o = opts(false, ColorMode::Always);
        render(&mut buf, &f, &o).unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert!(s.contains('\u{1b}'));
    }

    #[test]
    fn always_colors_with_icons() {
        let f = Node::new_file("main.rs", 1);
        let mut buf = Vec::new();
        let o = opts(true, ColorMode::Always);
        render(&mut buf, &f, &o).unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert!(s.contains('\u{e7a8}'));
        assert!(s.contains('\u{1b}'));
    }
}
