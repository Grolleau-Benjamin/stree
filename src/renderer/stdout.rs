use crate::model::node::Node;
use std::io::{self, Write};

pub fn render(root: &Node) {
    let mut out = io::stdout().lock();
    let _ = render_to(&mut out, root);
}

pub fn render_to<W: Write>(mut w: W, root: &Node) -> io::Result<()> {
    writeln!(w, "{}", root.name)?;
    let children = root.children_slice();
    let last_idx = children.len().saturating_sub(1);
    for (i, child) in children.iter().enumerate() {
        render_node_to(&mut w, child, "", i == last_idx)?;
    }
    Ok(())
}

fn render_node_to<W: Write>(w: &mut W, node: &Node, prefix: &str, is_last: bool) -> io::Result<()> {
    let branch = if is_last { "└── " } else { "├── " };
    writeln!(w, "{}{}{}", prefix, branch, node.name)?;

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
