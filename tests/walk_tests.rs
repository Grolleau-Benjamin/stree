use std::fs::{self, File};
use std::path::Path;
use tempfile::TempDir;

use arbor::config::WalkOptions;
use arbor::fs_scan::walk::walk_path;
use arbor::model::node::Kind;

fn make_fs_tree() -> (TempDir, std::path::PathBuf) {
    let tmp = TempDir::new().expect("tmpdir");
    let root = tmp.path().to_path_buf();

    fs::create_dir(root.join("src")).unwrap();
    fs::write(root.join("src/mod.rs"), b"mod test;").unwrap();

    fs::create_dir_all(root.join("dir/sub")).unwrap();
    fs::write(root.join("dir/sub/file.txt"), b"hello").unwrap();

    fs::write(root.join(".hidden"), b"dot").unwrap();

    fs::create_dir_all(root.join(".git/objects/aa")).unwrap();
    fs::write(root.join(".git/HEAD"), b"ref: refs/heads/main\n").unwrap();

    fs::create_dir_all(root.join("target")).unwrap();
    fs::write(root.join("target/bin.o"), b"\x00\x01").unwrap();

    fs::write(root.join(".gitignore"), b"/target\n").unwrap();

    (tmp, root)
}

fn find_child<'a>(
    node: &'a arbor::model::node::Node,
    name: &str,
) -> Option<&'a arbor::model::node::Node> {
    let children = node.children.as_deref().unwrap_or(&[]);
    children.iter().find(|n| n.name == name)
}

fn list_top_level(node: &arbor::model::node::Node) -> Vec<String> {
    node.children
        .as_deref()
        .unwrap_or(&[])
        .iter()
        .map(|n| n.name.clone())
        .collect()
}

#[test]
fn default_hides_dotfiles_and_gitignored() {
    let (_tmp, root) = make_fs_tree();

    let opts = WalkOptions {
        include_hidden: false,
        follow_gitignore: true,
        depth: None,
    };

    let tree = walk_path(Path::new(&root), &opts).expect("walk");
    let top = list_top_level(&tree);

    assert!(top.contains(&"src".to_string()));
    assert!(top.contains(&"dir".to_string()));

    assert!(!top.contains(&".git".to_string())); // hidden
    assert!(!top.contains(&".hidden".to_string())); // hidden
    assert!(!top.contains(&"target".to_string())); // ignored by .gitignore
}

#[test]
fn hidden_files_shows_dot_git_but_does_not_descend() {
    let (_tmp, root) = make_fs_tree();
    let opts = WalkOptions {
        include_hidden: true,
        follow_gitignore: true,
        depth: None,
    };

    let tree = walk_path(Path::new(&root), &opts).expect("walk");
    let top = list_top_level(&tree);

    assert!(top.contains(&".git".to_string()));
    assert!(!top.contains(&"target".to_string()));

    let git = find_child(&tree, ".git").expect(".git should exist");
    let git_kids = git.children.as_deref().unwrap_or(&[]);
    assert!(
        git_kids.is_empty(),
        ".git must be shown as a leaf (no descent into its contents)"
    );

    assert!(top.contains(&".hidden".to_string()));
}

#[test]
fn show_gitignored_entries_when_gitignore_flag_is_on() {
    let (_tmp, root) = make_fs_tree();

    let opts = WalkOptions {
        include_hidden: false,
        follow_gitignore: false,
        depth: None,
    };

    let tree = walk_path(Path::new(&root), &opts).expect("walk");
    let top = list_top_level(&tree);

    assert!(top.contains(&"target".to_string()));

    assert!(!top.contains(&".git".to_string()));
    assert!(!top.contains(&".hidden".to_string()));
}

#[test]
fn hidden_plus_gitignore_flag_shows_both_dot_git_and_target() {
    let (_tmp, root) = make_fs_tree();

    let opts = WalkOptions {
        include_hidden: true,    // show dotfiles
        follow_gitignore: false, // show gitignored
        depth: None,
    };

    let tree = walk_path(Path::new(&root), &opts).expect("walk");
    let top = list_top_level(&tree);

    assert!(top.contains(&".git".to_string()));
    assert!(top.contains(&"target".to_string()));

    let git = find_child(&tree, ".git").expect(".git should exist");
    assert!(git.children.as_deref().unwrap_or(&[]).is_empty());
}

#[test]
fn deep_structure_is_preserved() {
    let (_tmp, root) = make_fs_tree();

    let opts = WalkOptions {
        include_hidden: false,
        follow_gitignore: true,
        depth: None,
    };

    let tree = walk_path(Path::new(&root), &opts).expect("walk");

    let dir = find_child(&tree, "dir").expect("dir");
    let sub = find_child(dir, "sub").expect("sub under dir");
    let kids = sub.children.as_deref().unwrap_or(&[]);
    assert!(
        kids.iter().any(|n| n.name == "file.txt"),
        "file.txt must exist under dir/sub"
    );
}

#[test]
fn walk_with_no_depth_includes_all() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    // Create a nested folder structure
    let level1 = root.join("level1");
    let level2 = level1.join("level2");
    fs::create_dir_all(&level2).unwrap();

    File::create(level2.join("file.txt")).unwrap();

    let opts = WalkOptions {
        include_hidden: false,
        follow_gitignore: true,
        depth: None,
    };

    let tree = walk_path(root, &opts).unwrap();

    // Root is directory
    assert_eq!(tree.meta.kind, Kind::Dir);
    // Check that level1 exists
    let level1_node = tree
        .children
        .as_ref()
        .unwrap()
        .iter()
        .find(|n| n.name == "level1")
        .unwrap();
    assert_eq!(level1_node.meta.kind, Kind::Dir);
    // Check that level2 exists
    let level2_node = level1_node
        .children
        .as_ref()
        .unwrap()
        .iter()
        .find(|n| n.name == "level2")
        .unwrap();
    assert_eq!(level2_node.meta.kind, Kind::Dir);
    // Check that file exists
    let file_node = level2_node
        .children
        .as_ref()
        .unwrap()
        .iter()
        .find(|n| n.name == "file.txt")
        .unwrap();
    assert_eq!(file_node.meta.kind, Kind::File);
}

#[test]
fn walk_with_depth_limits_traversal() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    // Nested structure
    let level1 = root.join("level1");
    let level2 = level1.join("level2");
    fs::create_dir_all(&level2).unwrap();

    File::create(level2.join("file.txt")).unwrap();

    let opts = WalkOptions {
        include_hidden: false,
        follow_gitignore: true,
        depth: Some(1),
    };

    let tree = walk_path(root, &opts).unwrap();

    // Root should have children
    assert!(!tree.children.as_ref().unwrap().is_empty());

    // level1 should exist
    let level1_node = tree
        .children
        .as_ref()
        .unwrap()
        .iter()
        .find(|n| n.name == "level1")
        .unwrap();
    assert_eq!(level1_node.meta.kind, Kind::Dir);

    // level2 should NOT exist
    let level2_exists = level1_node
        .children
        .as_ref()
        .map(|c| c.iter().any(|n| n.name == "level2"))
        .unwrap_or(false);
    assert!(
        !level2_exists,
        "level2 should be skipped due to depth limit"
    );
}
