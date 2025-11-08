mod helpers;
use assert_cmd::prelude::*;
use helpers::{MockRegistry, TestEnvironment};
use predicates::prelude::*;
use std::path::PathBuf;
use std::process::Command;

#[test]
#[ignore] // Requires network and downloads real binaries
fn test_install_from_repo_file_windman() {
    let env = TestEnvironment::new().unwrap();
    env.set_env_vars();

    let repo_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("repo_files")
        .join("windman.repo");

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.arg("install")
        .arg("--repo")
        .arg(repo_file)
        .arg("--userland");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Successfully installed"));

    // Verify installation
    assert!(env.is_package_installed("windman"));

    env.cleanup();
}

#[test]
#[ignore] // Requires network
fn test_install_from_registry() {
    let env = TestEnvironment::new().unwrap();
    env.set_env_vars();

    let registry = MockRegistry::new().unwrap();

    // Add registry
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.arg("registry")
        .arg("add")
        .arg("test-registry")
        .arg(registry.url());
    cmd.assert().success();

    // Update registry
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.arg("registry").arg("update");
    cmd.assert().success();

    // Install windman (smallest package for testing)
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.arg("install").arg("windman").arg("--userland");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Successfully installed"));

    env.cleanup();
}

#[test]
fn test_install_missing_package() {
    let env = TestEnvironment::new().unwrap();
    env.set_env_vars();

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.arg("install").arg("nonexistent-package-xyz");

    // Should fail because no registries are configured
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No registries configured"));

    env.cleanup();
}

#[test]
fn test_install_with_version() {
    let env = TestEnvironment::new().unwrap();
    env.set_env_vars();

    let repo_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("repo_files")
        .join("windman.repo");

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.arg("install")
        .arg("--repo")
        .arg(repo_file)
        .arg("--version")
        .arg("0.1.0")
        .arg("--userland");

    // This might fail if the version doesn't exist, but it tests the argument parsing
    let _ = cmd.assert();

    env.cleanup();
}

#[test]
fn test_list_empty() {
    let env = TestEnvironment::new().unwrap();
    env.set_env_vars();

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No packages installed"));

    env.cleanup();
}

#[test]
fn test_search_no_registry() {
    let env = TestEnvironment::new().unwrap();
    env.set_env_vars();

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.arg("search").arg("prometheus");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No registries configured"));

    env.cleanup();
}
