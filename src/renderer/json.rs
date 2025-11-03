// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2025 Benjamin Grolleau and Angelo Tunney

use crate::model::node::{GitState, Kind, Node};
use serde::Serialize;
use std::io;

#[derive(Serialize)]
struct JsonNode<'a> {
    name: &'a str,
    kind: &'static str,
    size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    git: Option<&'static str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    children: Vec<JsonNode<'a>>,
}

fn kind_as_str(k: Kind) -> &'static str {
    match k {
        Kind::Dir => "dir",
        Kind::File => "file",
    }
}

fn git_as_str(g: GitState) -> &'static str {
    match g {
        GitState::Clean => "clean",
        GitState::Modified => "modified",
        GitState::Staged => "staged",
        GitState::Untracked => "untracked",
        GitState::Ignored => "ignored",
        GitState::Renamed => "renamed",
        GitState::Deleted => "deleted",
    }
}

impl<'a> From<&'a Node> for JsonNode<'a> {
    fn from(n: &'a Node) -> Self {
        JsonNode {
            name: &n.name,
            kind: kind_as_str(n.meta.kind),
            size: n.meta.size,
            git: n.meta.git.map(git_as_str),
            children: n.children_slice().iter().map(JsonNode::from).collect(),
        }
    }
}

pub fn render<W: io::Write>(w: W, root: &Node) -> io::Result<()> {
    let j = JsonNode::from(root);
    serde_json::to_writer_pretty(w, &j).map_err(io::Error::other)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::node::{GitState, Node};

    #[test]
    fn render_single_file() {
        let root = Node::new_file("file.txt", 42);
        let mut buf = Vec::new();

        render(&mut buf, &root).unwrap();
        let out = String::from_utf8(buf).unwrap();

        assert!(out.contains("\"name\": \"file.txt\""));
        assert!(out.contains("\"kind\": \"file\""));
        assert!(out.contains("\"size\": 42"));
        assert!(!out.contains("\"git\""));
        assert!(!out.contains("\"children\""));
    }

    #[test]
    fn render_directory_with_children() {
        let root = Node::new_dir(
            "root",
            vec![
                Node::new_file("a.txt", 1),
                Node::new_dir("src", vec![Node::new_file("main.rs", 10)]),
            ],
        );

        let mut buf = Vec::new();
        render(&mut buf, &root).unwrap();
        let out = String::from_utf8(buf).unwrap();

        assert!(out.contains("\"name\": \"root\""));
        assert!(out.contains("\"name\": \"a.txt\""));
        assert!(out.contains("\"name\": \"src\""));
        assert!(out.contains("\"name\": \"main.rs\""));
        assert!(out.contains("\"kind\": \"dir\""));
    }

    #[test]
    fn render_with_git_state() {
        let mut file = Node::new_file("foo.rs", 0);
        file.meta.git = Some(GitState::Modified);
        let mut buf = Vec::new();
        render(&mut buf, &file).unwrap();
        let out = String::from_utf8(buf).unwrap();

        assert!(out.contains("\"git\": \"modified\""));
    }

    #[test]
    fn render_nested_empty_children_skipped() {
        let root = Node::new_dir("root", vec![Node::new_dir("empty", vec![])]);
        let mut buf = Vec::new();
        render(&mut buf, &root).unwrap();
        let out = String::from_utf8(buf).unwrap();

        assert!(out.contains("\"name\": \"empty\""));
        assert!(!out.contains("\"children\": []"));
    }

    #[test]
    fn render_valid_json_output() {
        let root = Node::new_dir("r", vec![Node::new_file("a", 1)]);
        let mut buf = Vec::new();
        render(&mut buf, &root).unwrap();
        let text = String::from_utf8(buf).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert_eq!(parsed["name"], "r");
        assert_eq!(parsed["children"][0]["name"], "a");
        assert_eq!(parsed["children"][0]["kind"], "file");
    }
}
