use std::fs;
use std::path::Path;
use tempfile::TempDir;

use stree::config::WalkOptions;
use stree::fs_scan::walk::walk_path;

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
    node: &'a stree::model::node::Node,
    name: &str,
) -> Option<&'a stree::model::node::Node> {
    let children = node.children.as_deref().unwrap_or(&[]);
    children.iter().find(|n| n.name == name)
}

fn list_top_level(node: &stree::model::node::Node) -> Vec<String> {
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
        dirs_only: false,
        files_only: false,
        prune_empty: false,
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
        dirs_only: false,
        files_only: false,
        prune_empty: false,
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
        dirs_only: false,
        files_only: false,
        prune_empty: false,
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
        dirs_only: false,
        files_only: false,
        prune_empty: false,
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
        dirs_only: false,
        files_only: false,
        prune_empty: false,
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
