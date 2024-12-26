use clap::CommandFactory;
use clap_complete::{generate_to, Shell};
use clap_complete_nushell::Nushell;
use std::io::{self, Result};

include!("src/cli.rs");

fn main() -> io::Result<()> {
    let hash = commit_hash();
    println!("cargo:rerun-if-env-changed=COMMIT_HASH");
    println!("cargo:rustc-env=COMMIT_HASH={}", hash);
    // Tell Cargo to rerun this script if `build.rs` changes
    // Since we are generating completions in the package directory, we need to
    // set this so that Cargo doesn't rebuild every time.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=templates/");
    println!("cargo:rerun-if-changed=tests/");
    generate_completions()
}

fn commit_hash() -> String {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();
    String::from_utf8(output.stdout).unwrap()
}

fn generate_completions() -> Result<()> {
    const BIN_NAME: &str = "mc";
    const OUT_DIR: &str = "completions";

    let mut cmd = Args::command();

    for shell in [Shell::Bash, Shell::Zsh, Shell::Fish, Shell::Elvish] {
        generate_to(shell, &mut cmd, BIN_NAME, OUT_DIR)?;
    }

    generate_to(Nushell, &mut cmd, BIN_NAME, OUT_DIR)?;

    Ok(())
}
