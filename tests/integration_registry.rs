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

// Tests for 'ora registry sync' command (Issue #2)
#[test]
fn test_registry_sync_all() {
    let env = TestEnvironment::new().unwrap();
    let registry = MockRegistry::new().unwrap();

    // Add a registry
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.env("ORA_CONFIG_DIR", env.config_dir())
        .env("ORA_CACHE_DIR", env.cache_dir())
        .env("ORA_DATA_DIR", env.data_dir())
        .arg("registry")
        .arg("add")
        .arg("test-registry")
        .arg(registry.url());
    cmd.assert().success();

    // Sync all registries
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.env("ORA_CONFIG_DIR", env.config_dir())
        .env("ORA_CACHE_DIR", env.cache_dir())
        .env("ORA_DATA_DIR", env.data_dir())
        .arg("registry")
        .arg("sync");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Syncing"))
        .stdout(predicate::str::contains("test-registry"));
}

#[test]
fn test_registry_sync_specific() {
    let env = TestEnvironment::new().unwrap();
    let registry = MockRegistry::new().unwrap();

    // Add a registry
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.env("ORA_CONFIG_DIR", env.config_dir())
        .env("ORA_CACHE_DIR", env.cache_dir())
        .env("ORA_DATA_DIR", env.data_dir())
        .arg("registry")
        .arg("add")
        .arg("test-registry")
        .arg(registry.url());
    cmd.assert().success();

    // Sync specific registry
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.env("ORA_CONFIG_DIR", env.config_dir())
        .env("ORA_CACHE_DIR", env.cache_dir())
        .env("ORA_DATA_DIR", env.data_dir())
        .arg("registry")
        .arg("sync")
        .arg("test-registry");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Syncing registry: test-registry"))
        .stdout(predicate::str::contains("synced successfully"));
}

#[test]
fn test_registry_sync_nonexistent() {
    let env = TestEnvironment::new().unwrap();

    // Try to sync a non-existent registry
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.env("ORA_CONFIG_DIR", env.config_dir())
        .env("ORA_CACHE_DIR", env.cache_dir())
        .env("ORA_DATA_DIR", env.data_dir())
        .arg("registry")
        .arg("sync")
        .arg("nonexistent-registry");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn test_registry_sync_no_registries() {
    let env = TestEnvironment::new().unwrap();

    // Try to sync when no registries are configured
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.env("ORA_CONFIG_DIR", env.config_dir())
        .env("ORA_CACHE_DIR", env.cache_dir())
        .env("ORA_DATA_DIR", env.data_dir())
        .arg("registry")
        .arg("sync");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No registries configured"));
}

// Tests for 'ora registry verify' command (Issue #1)
#[test]
fn test_registry_verify_valid() {
    let env = TestEnvironment::new().unwrap();
    let registry = MockRegistry::new().unwrap();

    // Add a registry (which automatically syncs it)
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.env("ORA_CONFIG_DIR", env.config_dir())
        .env("ORA_CACHE_DIR", env.cache_dir())
        .env("ORA_DATA_DIR", env.data_dir())
        .arg("registry")
        .arg("add")
        .arg("test-registry")
        .arg(registry.url());
    cmd.assert().success();

    // Verify the registry
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.env("ORA_CONFIG_DIR", env.config_dir())
        .env("ORA_CACHE_DIR", env.cache_dir())
        .env("ORA_DATA_DIR", env.data_dir())
        .arg("registry")
        .arg("verify")
        .arg("test-registry");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Verifying registry: test-registry"))
        .stdout(predicate::str::contains("✓ Registry found in configuration"))
        .stdout(predicate::str::contains("✓ Registry synced locally"))
        .stdout(predicate::str::contains("✓ Valid git repository"))
        .stdout(predicate::str::contains("✓ 'packages/' directory exists"))
        .stdout(predicate::str::contains("verification complete"));
}

#[test]
fn test_registry_verify_nonexistent() {
    let env = TestEnvironment::new().unwrap();

    // Try to verify a non-existent registry
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.env("ORA_CONFIG_DIR", env.config_dir())
        .env("ORA_CACHE_DIR", env.cache_dir())
        .env("ORA_DATA_DIR", env.data_dir())
        .arg("registry")
        .arg("verify")
        .arg("nonexistent-registry");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not found in configuration"));
}

#[test]
fn test_registry_verify_not_synced() {
    let env = TestEnvironment::new().unwrap();
    let registry = MockRegistry::new().unwrap();

    // Add a registry but then remove its local directory
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.env("ORA_CONFIG_DIR", env.config_dir())
        .env("ORA_CACHE_DIR", env.cache_dir())
        .env("ORA_DATA_DIR", env.data_dir())
        .arg("registry")
        .arg("add")
        .arg("test-registry")
        .arg(registry.url());
    cmd.assert().success();

    // Remove the synced directory manually
    let registry_path = env.cache_dir().join("registries/test-registry");
    let _ = std::fs::remove_dir_all(&registry_path);

    // Try to verify - should fail with "not synced" message
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.env("ORA_CONFIG_DIR", env.config_dir())
        .env("ORA_CACHE_DIR", env.cache_dir())
        .env("ORA_DATA_DIR", env.data_dir())
        .arg("registry")
        .arg("verify")
        .arg("test-registry");

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("✗ Registry not synced locally"))
        .stdout(predicate::str::contains("ora registry sync test-registry"));
}

#[test]
fn test_registry_verify_shows_package_count() {
    let env = TestEnvironment::new().unwrap();
    let registry = MockRegistry::new().unwrap();

    // Add a registry
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.env("ORA_CONFIG_DIR", env.config_dir())
        .env("ORA_CACHE_DIR", env.cache_dir())
        .env("ORA_DATA_DIR", env.data_dir())
        .arg("registry")
        .arg("add")
        .arg("test-registry")
        .arg(registry.url());
    cmd.assert().success();

    // Verify and check that it shows package count
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.env("ORA_CONFIG_DIR", env.config_dir())
        .env("ORA_CACHE_DIR", env.cache_dir())
        .env("ORA_DATA_DIR", env.data_dir())
        .arg("registry")
        .arg("verify")
        .arg("test-registry");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Found").and(predicate::str::contains("package")));
}
