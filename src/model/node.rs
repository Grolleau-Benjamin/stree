#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Dir,
    File,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitState {
    Clean,
    Modified,
    Staged,
    Untracked,
    Ignored,
    Renamed,
    Deleted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetaData {
    pub kind: Kind,
    pub size: Option<u64>,
    pub git: Option<GitState>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub name: String,
    pub meta: MetaData,
    pub children: Option<Vec<Node>>,
}

impl Node {
    pub fn new_file(name: &str, size: u64) -> Self {
        Self {
            name: name.to_string(),
            meta: MetaData {
                kind: Kind::File,
                size: Some(size),
                git: None,
            },
            children: None,
        }
    }

    pub fn new_dir(name: &str, children: Vec<Node>) -> Self {
        Self {
            name: name.to_string(),
            meta: MetaData {
                kind: Kind::Dir,
                size: None,
                git: None,
            },
            children: Some(children),
        }
    }

    pub fn children_slice(&self) -> &[Node] {
        self.children.as_deref().unwrap_or(&[])
    }

    pub fn is_dir(&self) -> bool {
        self.meta.kind == Kind::Dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_file() {
        let node = Node::new_file("file.txt", 42);
        assert_eq!(node.name, "file.txt");
        assert_eq!(node.meta.kind, Kind::File);
        assert_eq!(node.meta.size, Some(42));
        assert!(node.children.is_none());
    }

    #[test]
    fn test_new_dir() {
        let child = Node::new_file("main.rs", 100);
        let dir = Node::new_dir("src", vec![child.clone()]);
        assert_eq!(dir.name, "src");
        assert_eq!(dir.meta.kind, Kind::Dir);
        assert!(dir.meta.size.is_none());
        assert_eq!(dir.children.unwrap(), vec![child]);
    }
}
