mod helpers;
use assert_cmd::prelude::*;
use helpers::{MockRegistry, TestEnvironment};
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_registry_add() {
    let _env = TestEnvironment::new().unwrap();
    let registry = MockRegistry::new().unwrap();

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.arg("registry")
        .arg("add")
        .arg("test-registry")
        .arg(registry.url());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("added successfully"));
}

#[test]
fn test_registry_list() {
    let _env = TestEnvironment::new().unwrap();
    let registry = MockRegistry::new().unwrap();

    // First add a registry
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.arg("registry")
        .arg("add")
        .arg("test-registry")
        .arg(registry.url());
    cmd.assert().success();

    // Then list registries
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.arg("registry").arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test-registry"));
}

#[test]
fn test_registry_update() {
    let _env = TestEnvironment::new().unwrap();
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

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test-registry").or(predicate::str::contains("Updating")));
}

#[test]
fn test_registry_remove() {
    let _env = TestEnvironment::new().unwrap();
    let registry = MockRegistry::new().unwrap();

    // Add registry
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.arg("registry")
        .arg("add")
        .arg("test-registry")
        .arg(registry.url());
    cmd.assert().success();

    // Remove registry
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.arg("registry").arg("remove").arg("test-registry");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("removed"));
}

#[test]
fn test_mock_registry_has_packages() {
    let registry = MockRegistry::new().unwrap();
    let packages = registry.list_packages().unwrap();

    assert!(!packages.is_empty(), "Registry should have test packages");
    assert!(
        packages.contains(&"windman".to_string()),
        "Registry should contain windman"
    );
    assert!(
        packages.contains(&"prometheus".to_string()),
        "Registry should contain prometheus"
    );
}
