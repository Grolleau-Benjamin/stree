use std::fs;
use std::fs::File;
use std::io::Write;
use stree::fs_scan::walk::walk_path;
use stree::model::node::Kind;

#[test]
fn walk_simple_tree() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    let src = root.join("src");
    fs::create_dir(&src).unwrap();

    let mut f = File::create(src.join("main.rs")).unwrap();
    writeln!(f, "fn main() {{}}").unwrap();

    let tree = walk_path(root).unwrap();
    assert_eq!(tree.meta.kind, Kind::Dir);
    let src_node = tree
        .children
        .as_ref()
        .unwrap()
        .iter()
        .find(|n| n.name == "src")
        .unwrap();
    assert_eq!(src_node.meta.kind, Kind::Dir);
    let names: Vec<_> = src_node
        .children
        .as_ref()
        .unwrap()
        .iter()
        .map(|n| n.name.as_str())
        .collect();
    assert!(names.contains(&"main.rs"));
}
