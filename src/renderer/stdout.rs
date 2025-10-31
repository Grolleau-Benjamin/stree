// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2025 Benjamin Grolleau and Angelo Tunney

use crate::{config::RenderOptions, model::node::Node, renderer::icons};
use std::io::{self, Write};

pub fn render(root: &Node, opts: &RenderOptions) {
    let mut out = io::stdout().lock();

    let plain_name = |n: &Node| {
        if n.is_dir() {
            format!("{}/", n.name)
        } else {
            n.name.clone()
        }
    };

    let icon_name = |n: &Node| {
        if n.is_dir() {
            format!("{} {}/", icons::dir_icon(&n.name), n.name)
        } else {
            format!("{} {}", icons::file_icon(&n.name), n.name)
        }
    };

    let make_name: &dyn Fn(&Node) -> String = if opts.icons { &icon_name } else { &plain_name };

    let _ = render_to(&mut out, root, make_name);
}

pub fn render_to<W: Write>(
    mut w: W,
    root: &Node,
    make_name: &dyn Fn(&Node) -> String,
) -> io::Result<()> {
    writeln!(w, "{}", make_name(root))?;
    let children = root.children_slice();
    let last_idx = children.len().saturating_sub(1);
    for (i, child) in children.iter().enumerate() {
        render_node_to(&mut w, child, "", i == last_idx, make_name)?;
    }
    Ok(())
}

fn render_node_to<W: Write>(
    w: &mut W,
    node: &Node,
    prefix: &str,
    is_last: bool,
    make_name: &dyn Fn(&Node) -> String,
) -> io::Result<()> {
    let branch = if is_last { "└── " } else { "├── " };
    writeln!(w, "{}{}{}", prefix, branch, make_name(node))?;

    let mut new_prefix = String::with_capacity(prefix.len() + 4);
    new_prefix.push_str(prefix);
    new_prefix.push_str(if is_last { "    " } else { "│   " });

    let children = node.children_slice();
    let last_idx = children.len().saturating_sub(1);
    for (i, child) in children.iter().enumerate() {
        render_node_to(w, child, &new_prefix, i == last_idx, make_name)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::node::Node;

    #[test]
    fn renders_single_node() {
        let root = Node::new_dir("root", vec![]);
        let mut buf = Vec::new();

        let plain_name = |n: &Node| {
            if n.is_dir() {
                format!("{}/", n.name)
            } else {
                n.name.clone()
            }
        };

        render_to(&mut buf, &root, &plain_name).unwrap();

        assert_eq!(String::from_utf8(buf).unwrap(), "root/\n");
    }

    #[test]
    fn renders_tree_with_dirs_and_files() {
        // root
        // ├── src
        // │   ├── main.rs
        // │   └── lib.rs
        // └── README.md
        let root = Node::new_dir(
            "root",
            vec![
                Node::new_dir(
                    "src",
                    vec![
                        Node::new_file("main.rs", 123),
                        Node::new_file("lib.rs", 456),
                    ],
                ),
                Node::new_file("README.md", 789),
            ],
        );

        let mut buf = Vec::new();

        let plain_name = |n: &Node| {
            if n.is_dir() {
                format!("{}/", n.name)
            } else {
                n.name.clone()
            }
        };

        render_to(&mut buf, &root, &plain_name).unwrap();
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
    fn renders_nested_last_branch_correctly() {
        // root
        // └── a
        //     └── b
        //         └── c.txt
        let root = Node::new_dir(
            "root",
            vec![Node::new_dir(
                "a",
                vec![Node::new_dir("b", vec![Node::new_file("c.txt", 1)])],
            )],
        );

        let mut buf = Vec::new();

        let plain_name = |n: &Node| {
            if n.is_dir() {
                format!("{}/", n.name)
            } else {
                n.name.clone()
            }
        };

        render_to(&mut buf, &root, &plain_name).unwrap();
        let got = String::from_utf8(buf).unwrap();

        let expected = "\
root/
└── a/
    └── b/
        └── c.txt
";
        assert_eq!(got, expected);
    }

    #[test]
    fn renders_single_node_with_icons() {
        let root = Node::new_dir("root", vec![]);
        let mut buf = Vec::new();

        let icon_name = |n: &Node| {
            let icon = if n.is_dir() {
                icons::dir_icon(&n.name)
            } else {
                icons::file_icon(&n.name)
            };
            if n.is_dir() {
                format!("{} {}/", icon, n.name)
            } else {
                format!("{} {}", icon, n.name)
            }
        };

        render_to(&mut buf, &root, &icon_name).unwrap();
        let got = String::from_utf8(buf).unwrap();

        let expected = format!("{}\n", format!("{}\u{0020}root/", '\u{f115}'));
        assert_eq!(got, expected);
    }

    #[test]
    fn renders_tree_with_icons_dir_exact_file_and_extension() {
        // root
        // ├── src
        // │   ├── main.rs
        // │   └── lib.rs
        // └── README.md
        let root = Node::new_dir(
            "root",
            vec![
                Node::new_dir(
                    "src",
                    vec![
                        Node::new_file("main.rs", 123),
                        Node::new_file("lib.rs", 456),
                    ],
                ),
                Node::new_file("README.md", 789),
            ],
        );

        let mut buf = Vec::new();

        let icon_name = |n: &Node| {
            let icon = if n.is_dir() {
                icons::dir_icon(&n.name)
            } else {
                icons::file_icon(&n.name)
            };
            if n.is_dir() {
                format!("{} {}/", icon, n.name)
            } else {
                format!("{} {}", icon, n.name)
            }
        };

        render_to(&mut buf, &root, &icon_name).unwrap();
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
    fn renders_unknowns_with_default_icons() {
        // root
        // ├── foo
        // └── unknown.xyz
        let root = Node::new_dir(
            "root",
            vec![Node::new_dir("foo", vec![]), Node::new_file("ben.grl", 1)],
        );

        let mut buf = Vec::new();

        let icon_name = |n: &Node| {
            let icon = if n.is_dir() {
                icons::dir_icon(&n.name)
            } else {
                icons::file_icon(&n.name)
            };
            if n.is_dir() {
                format!("{} {}/", icon, n.name)
            } else {
                format!("{} {}", icon, n.name)
            }
        };

        render_to(&mut buf, &root, &icon_name).unwrap();
        let got = String::from_utf8(buf).unwrap();

        let expected = "\
\u{f115} root/
├── \u{f115} foo/
└── \u{f016} ben.grl
";
        assert_eq!(got, expected);
    }
}
