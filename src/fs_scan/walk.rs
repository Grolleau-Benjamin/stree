use std::{fs, io, path::Path};

use crate::model::node::Node;

pub fn walk_path(root: &Path) -> io::Result<Node> {
    let md = fs::symlink_metadata(root)?;
    let name = root
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| ".".into());

    if md.is_file() {
        return Ok(Node::new_file(&name, md.len()));
    }

    if md.is_dir() {
        let mut children = Vec::new();
        for entry in fs::read_dir(root)? {
            let entry = entry?;
            let child = walk_path(&entry.path())?;
            children.push(child);
        }
        return Ok(Node::new_dir(&name, children));
    }

    Ok(Node::new_file(&name, 0))
}
