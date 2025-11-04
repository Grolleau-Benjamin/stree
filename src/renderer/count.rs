// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2025 Benjamin Grolleau and Angelo Tunney

use crate::model::node::{Kind, Node};
use std::io;

pub fn render<W: io::Write>(mut w: W, root: &Node) -> io::Result<()> {
    let mut nb_files: usize = 0;
    let mut nb_dirs: usize = 0;

    walk(root, &mut nb_files, &mut nb_dirs);

    writeln!(
        w,
        "\u{f115} Directories: {} | \u{f016} Files: {}",
        nb_dirs, nb_files
    )?;
    Ok(())
}

fn walk(n: &Node, nb_files: &mut usize, nb_dirs: &mut usize) {
    match n.meta.kind {
        Kind::File => *nb_files += 1,
        Kind::Dir => *nb_dirs += 1,
    };

    if let Some(children) = &n.children {
        for child in children {
            walk(child, nb_files, nb_dirs);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::node::Node;

    fn s(v: &[u8]) -> String {
        String::from_utf8(v.to_vec()).expect("utf8")
    }

    #[test]
    fn counts_on_empty_dir() {
        let root = Node::new_dir("root", vec![]);

        let mut buf: Vec<u8> = Vec::new();
        let res = render(&mut buf, &root);
        assert!(res.is_ok());

        let expected = format!("\u{f115} Directories: {} | \u{f016} Files: {}\n", 1, 0);
        assert_eq!(s(&buf), expected);
    }

    #[test]
    fn counts_on_mixed_tree() {
        // root
        // ├── a.txt
        // ├── b.txt
        // └── sub/
        //     └── c.txt
        let sub = Node::new_dir("sub", vec![Node::new_file("c.txt", 3)]);
        let root = Node::new_dir(
            "root",
            vec![Node::new_file("a.txt", 1), Node::new_file("b.txt", 2), sub],
        );

        let mut buf: Vec<u8> = Vec::new();
        let res = render(&mut buf, &root);
        assert!(res.is_ok());

        let expected = format!("\u{f115} Directories: {} | \u{f016} Files: {}\n", 2, 3);
        assert_eq!(s(&buf), expected);
    }

    #[test]
    fn counts_when_root_is_file() {
        let root = Node::new_file("lonely.txt", 1);

        let mut buf: Vec<u8> = Vec::new();
        let res = render(&mut buf, &root);
        assert!(res.is_ok());

        let expected = format!("\u{f115} Directories: {} | \u{f016} Files: {}\n", 0, 1);
        assert_eq!(s(&buf), expected);
    }
}
