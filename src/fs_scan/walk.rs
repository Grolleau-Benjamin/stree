use ignore::{DirEntry, WalkBuilder};
use std::{
    collections::HashMap,
    ffi::OsStr,
    io,
    path::{Path, PathBuf},
};

use crate::{config::WalkOptions, model::node::Node};

#[derive(Default, Debug)]
struct TmpNode {
    name: String,
    size: u64,
    is_dir: bool,
    children: Vec<usize>,
}

pub fn walk_path(root: &Path, opts: &WalkOptions) -> io::Result<Node> {
    let mut wb = WalkBuilder::new(root);
    wb.follow_links(false)
        .hidden(!opts.include_hidden)
        .git_ignore(opts.follow_gitignore)
        .git_exclude(opts.follow_gitignore)
        .git_global(opts.follow_gitignore)
        .threads(1)
        .filter_entry({
            let include_hidden = opts.include_hidden;
            move |e: &DirEntry| {
                if include_hidden {
                    if is_dot_git_dir(e) {
                        return true;
                    }
                    if has_dot_git_ancestor(e.path()) {
                        return false;
                    }
                }
                true
            }
        });

    let mut nodes_by_path: HashMap<PathBuf, usize> = HashMap::new();
    let mut arena: Vec<TmpNode> = Vec::new();

    let root_name = root
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| ".".into());

    let root_idx = push_dir(&mut arena, &root_name);
    nodes_by_path.insert(root.to_path_buf(), root_idx);

    for result in wb.build() {
        let entry = match result {
            Ok(e) => e,
            Err(_) => continue,
        };

        if entry.depth() == 0 {
            continue;
        }

        if should_skip(&entry) {
            continue;
        }

        let path = entry.path();
        let parent_path = match path.parent() {
            Some(p) => p,
            None => continue,
        };

        let parent_idx =
            ensure_dir_idx(parent_path, &mut nodes_by_path, &mut arena, root, root_idx);

        match entry.file_type() {
            Some(ft) if ft.is_dir() => {
                let name = file_name_str(path);
                let idx = push_dir(&mut arena, &name);
                nodes_by_path.insert(path.to_path_buf(), idx);
                push_child(parent_idx, idx, &mut arena);
            }
            Some(ft) if ft.is_file() => {
                let size = entry.metadata().ok().map(|m| m.len()).unwrap_or(0);
                let name = file_name_str(path);
                let idx = push_file(&mut arena, &name, size);
                push_child(parent_idx, idx, &mut arena);
            }
            _ => continue,
        }
    }

    Ok(materialize(root_idx, &arena))
}

fn should_skip(entry: &DirEntry) -> bool {
    entry.path().file_name().is_none()
}

fn file_name_str(path: &Path) -> String {
    path.file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default()
}

fn push_dir(arena: &mut Vec<TmpNode>, name: &str) -> usize {
    let n = TmpNode {
        name: name.to_string(),
        size: 0,
        is_dir: true,
        children: Vec::new(),
    };
    arena.push(n);
    arena.len() - 1
}

fn push_file(arena: &mut Vec<TmpNode>, name: &str, size: u64) -> usize {
    let n = TmpNode {
        name: name.to_string(),
        size,
        is_dir: false,
        children: Vec::new(),
    };
    arena.push(n);
    arena.len() - 1
}

fn push_child(parent_idx: usize, child_idx: usize, arena: &mut Vec<TmpNode>) {
    if let Some(parent) = arena.get_mut(parent_idx) {
        parent.children.push(child_idx);
    }
}

fn ensure_dir_idx(
    dir: &Path,
    nodes_by_path: &mut HashMap<PathBuf, usize>,
    arena: &mut Vec<TmpNode>,
    root: &Path,
    root_idx: usize,
) -> usize {
    if let Some(&idx) = nodes_by_path.get(dir) {
        return idx;
    }

    if dir == root {
        return root_idx;
    }

    if !dir.starts_with(root) {
        return root_idx;
    }

    let p = dir.parent().unwrap_or(root);
    let p_idx = ensure_dir_idx(p, nodes_by_path, arena, root, root_idx);

    let name = file_name_str(dir);
    let idx = push_dir(arena, &name);
    nodes_by_path.insert(dir.to_path_buf(), idx);
    push_child(p_idx, idx, arena);
    idx
}

fn materialize(idx: usize, arena: &Vec<TmpNode>) -> Node {
    let tmp = &arena[idx];
    if tmp.is_dir {
        let mut kids = Vec::with_capacity(tmp.children.len());
        for &c in &tmp.children {
            kids.push(materialize(c, arena));
        }
        Node::new_dir(&tmp.name, kids)
    } else {
        Node::new_file(&tmp.name, tmp.size)
    }
}

fn is_dot_git_dir(entry: &DirEntry) -> bool {
    entry.file_type().map(|t| t.is_dir()).unwrap_or(false)
        && entry.path().file_name() == Some(OsStr::new(".git"))
}

fn has_dot_git_ancestor(path: &std::path::Path) -> bool {
    let mut p = path;
    while let Some(parent) = p.parent() {
        if parent.file_name() == Some(OsStr::new(".git")) {
            return true;
        }
        p = parent;
    }
    false
}
