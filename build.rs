use chrono::Local;
use std::{env, io, process::Command};

fn main() -> io::Result<()> {
    let pkg_version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.0.0".to_string());
    let is_alpha = is_development_version(&pkg_version);
    let sha = git_hash().unwrap_or_else(|| "unknown".to_string());
    let date = build_date();
    let features = nonstandard_features_string();

    let short = pkg_version.clone();

    let mut version_line = format!("v{}", pkg_version);
    if !features.is_empty() {
        version_line.push_str(&format!(" [{}]", features));
    }

    let long = if is_alpha {
        format!("{version_line} [{sha}] built on {date} (alpha)")
    } else {
        format!("{version_line} [{sha}]")
    };

    println!("cargo:rustc-env=STREE_VERSION_SHORT={}", short);
    println!("cargo:rustc-env=STREE_VERSION_LONG={}", long);
    println!("cargo:rustc-env=STREE_GIT_SHA={}", sha);
    println!("cargo:rustc-env=STREE_BUILD_DATE={}", date);

    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/");
    println!("cargo:rerun-if-changed=.git/packed-refs");
    println!("cargo:rerun-if-env-changed=GITHUB_SHA");

    Ok(())
}

fn is_development_version(ver: &str) -> bool {
    ver.ends_with("-alpha") || ver.contains("-alpha.")
}

fn git_hash() -> Option<String> {
    if let Ok(out) = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        && out.status.success()
    {
        let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if !s.is_empty() {
            return Some(s);
        }
    }
    if let Ok(full) = env::var("GITHUB_SHA") {
        return Some(full.chars().take(7).collect());
    }
    None
}

fn nonstandard_features_string() -> String {
    let mut parts = Vec::new();
    parts.push(if feature_enabled("GIT") {
        "+git"
    } else {
        "-git"
    });
    parts.join(", ")
}

fn feature_enabled(name: &str) -> bool {
    env::var(format!("CARGO_FEATURE_{name}"))
        .map(|v| !v.is_empty())
        .unwrap_or(false)
}

fn build_date() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}
