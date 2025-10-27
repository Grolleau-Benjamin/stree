use crate::model::node::Node;

pub fn render(root: &Node) {
    println!("{}", root.name);

    let children = root.children_slice();
    let last_idx = children.len().saturating_sub(1);

    for (i, child) in children.iter().enumerate() {
        render_node(child, "", i == last_idx);
    }
}

fn render_node(node: &Node, prefix: &str, is_last: bool) {
    let branch = if is_last { "└── " } else { "├── " };
    println!("{}{}{}", prefix, branch, node.name);

    let new_prefix = if is_last {
        format!("{}    ", prefix)
    } else {
        format!("{}│   ", prefix)
    };

    let children = node.children_slice();
    let last_idx = children.len().saturating_sub(1);

    for (i, child) in children.iter().enumerate() {
        render_node(child, &new_prefix, i == last_idx);
    }
}
