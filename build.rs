#[cfg(feature = "completions")]
include!("src/cli.rs");

fn main() {
    let hash = commit_hash();
    println!("cargo:rerun-if-env-changed=COMMIT_HASH");
    println!("cargo:rustc-env=COMMIT_HASH={}", hash);
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/");

    #[cfg(feature = "completions")]
    generate_completions();
}

fn commit_hash() -> String {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();
    String::from_utf8(output.stdout).unwrap()
}

#[cfg(feature = "completions")]
fn generate_completions() {
    use clap::CommandFactory;
    use clap_complete::{generate_to, Shell};

    const BIN_NAME: &str = "mc";
    const OUT_DIR: &str = "completions";

    std::fs::create_dir_all(OUT_DIR).unwrap();

    let mut cmd = Args::command();

    for shell in [Shell::Bash, Shell::Zsh, Shell::Fish, Shell::Elvish] {
        generate_to(shell, &mut cmd, BIN_NAME, OUT_DIR).unwrap();
    }
    generate_to(clap_complete_nushell::Nushell, &mut cmd, BIN_NAME, OUT_DIR).unwrap();
}
