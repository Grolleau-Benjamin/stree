use crate::model::node::Node;
use std::io::{self, Write};

pub fn render(root: &Node) {
    let mut out = io::stdout().lock();
    let _ = render_to(&mut out, root);
}

pub fn render_to<W: Write>(mut w: W, root: &Node) -> io::Result<()> {
    let name = if root.is_dir() {
        format!("{}/", root.name)
    } else {
        root.name.clone()
    };
    writeln!(w, "{}", name)?;
    let children = root.children_slice();
    let last_idx = children.len().saturating_sub(1);
    for (i, child) in children.iter().enumerate() {
        render_node_to(&mut w, child, "", i == last_idx)?;
    }
    Ok(())
}

fn render_node_to<W: Write>(w: &mut W, node: &Node, prefix: &str, is_last: bool) -> io::Result<()> {
    let branch = if is_last { "└── " } else { "├── " };
    let name = if node.is_dir() {
        format!("{}/", node.name)
    } else {
        node.name.clone()
    };
    writeln!(w, "{}{}{}", prefix, branch, name)?;

    let mut new_prefix = String::with_capacity(prefix.len() + 4);
    new_prefix.push_str(prefix);
    new_prefix.push_str(if is_last { "    " } else { "│   " });

    let children = node.children_slice();
    let last_idx = children.len().saturating_sub(1);
    for (i, child) in children.iter().enumerate() {
        render_node_to(w, child, &new_prefix, i == last_idx)?;
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
        render_to(&mut buf, &root).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "root\n");
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
        render_to(&mut buf, &root).unwrap();
        let got = String::from_utf8(buf).unwrap();

        let expected = "\
root
├── src
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
        render_to(&mut buf, &root).unwrap();
        let got = String::from_utf8(buf).unwrap();

        let expected = "\
root
└── a
    └── b
        └── c.txt
";
        assert_eq!(got, expected);
    }
}
