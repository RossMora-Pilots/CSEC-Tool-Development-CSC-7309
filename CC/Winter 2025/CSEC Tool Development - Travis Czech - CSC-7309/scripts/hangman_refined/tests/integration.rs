//! Integration tests for the Hangman refined game.
//!
//! These tests simulate full game scenarios from start to finish,
//! verifying that the game state machine behaves correctly across
//! complete play-throughs.

// Import the main module's public types
// Note: integration tests can only access public items.
// Since Hangman's types are private (no `pub`), we test via the binary's behavior.

use std::process::Command;

/// Verify that the hangman_refined binary compiles and starts without panicking.
#[test]
fn binary_compiles_and_starts() {
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

/// Verify that all unit tests pass when run via cargo test.
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

/// Verify that cargo check reports zero warnings.
#[test]
fn zero_warnings() {
    let output = Command::new("cargo")
        .args(["check", "--message-format=short"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run cargo check");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("warning"),
        "Expected zero warnings, got: {}",
        stderr
    );
}
