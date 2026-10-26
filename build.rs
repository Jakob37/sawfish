fn main() {
    // The source archive has no Git worktree. Report the explicit local build version.
    println!("cargo:rustc-env=VERGEN_GIT_DESCRIBE={}", env!("CARGO_PKG_VERSION"));
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=src");
}
