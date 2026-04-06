//! Integration tests for the Guessing Game.
//!
//! Verifies that the guessing_game binary compiles cleanly
//! and that all unit tests pass.

use std::process::Command;

/// Verify that the guessing_game binary compiles successfully.
#[test]
fn binary_compiles() {
    let output = Command::new("cargo")
        .args(["build", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run cargo build");

    assert!(
        output.status.success(),
        "cargo build failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Verify that all unit tests pass.
#[test]
fn all_unit_tests_pass() {
    let output = Command::new("cargo")
        .args(["test", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run cargo test");

    assert!(
        output.status.success(),
        "cargo test failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
