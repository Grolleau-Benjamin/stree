// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2025 Benjamin Grolleau and Angelo Tunney

use crate::model::node::{GitState, Node};
use git2::{Repository, Status, StatusOptions};
use smol_str::SmolStr;
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;

pub type GitMap = HashMap<SmolStr, GitState>;

pub fn collect_git_states(root: &Path) -> GitMap {
    let Ok(repo) = Repository::discover(root) else {
        return HashMap::new();
    };

    // not a lot of case when you have more than 4096 files!
    let mut map = HashMap::with_capacity(4096);
    let mut opts = StatusOptions::new();
    opts.include_untracked(true)
        .include_ignored(true)
        .recurse_untracked_dirs(true);

    let index_mask = Status::INDEX_MODIFIED | Status::INDEX_NEW | Status::INDEX_RENAMED;

    if let Ok(statuses) = repo.statuses(Some(&mut opts)) {
        for entry in statuses.iter() {
            if let Some(path) = entry.path() {
                let s = entry.status();
                let state = match true {
                    _ if s.contains(Status::WT_MODIFIED) => GitState::Modified,
                    _ if s.intersects(index_mask) => GitState::Staged,
                    _ if s.contains(Status::WT_NEW) => GitState::Untracked,
                    _ if s.contains(Status::IGNORED) => GitState::Ignored,
                    _ if s.contains(Status::WT_DELETED) => GitState::Deleted,
                    _ => GitState::Clean,
                };
                map.insert(SmolStr::new(path), state);
            }
        }
    }

    map
}

pub fn enrich_with_git(node: &mut Node, git: &GitMap, buf: &mut String) {
    let keep = buf.len();
    if !buf.is_empty() {
        buf.push('/');
    }
    buf.push_str(&node.name);

    let lookup_key = buf.strip_prefix("./").unwrap_or(buf.as_str());

    if !node.is_dir()
        && let Some(&state) = git.get(lookup_key)
    {
        node.meta.git = Some(state);
    }

    if let Some(children) = node.children.as_mut() {
        for c in children {
            enrich_with_git(c, git, buf);
        }
    }

    buf.truncate(keep);
}

pub fn write_git_branch(out: &mut impl Write, root: &std::path::Path) {
    let Ok(repo) = Repository::discover(root) else {
        return;
    };

    let Ok(head) = repo.head() else {
        return;
    };

    let branch = if head.is_branch() {
        head.shorthand().unwrap_or("unknown")
    } else {
        "HEAD"
    };

    let _ = writeln!(out, "(⎇ {branch})");
}
