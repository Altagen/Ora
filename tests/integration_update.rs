use assert_cmd::Command;
use predicates::prelude::*;

mod helpers;
use helpers::test_env::TestEnvironment;

#[test]
fn test_update_nonexistent_package() {
    let env = TestEnvironment::new().unwrap();
    env.set_env_vars();

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.arg("update").arg("nonexistent-package");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));

    env.cleanup();
}

#[test]
fn test_update_all_no_packages() {
    let env = TestEnvironment::new().unwrap();
    env.set_env_vars();

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.arg("update").arg("--all");

    // Should succeed but report no packages to update
    cmd.assert().success();

    env.cleanup();
}

#[test]
fn test_update_migrates_schema_version() {
    let env = TestEnvironment::new().unwrap();

    // Create a fake installed package with old schema version (0.0)
    let installed_db_path = env.config_dir().join("installed.toml");
    let old_package_toml = r#"
[packages.test-package]
schema_version = "0.0"
name = "test-package"
version = "1.0.0"
installed_at = "2024-01-01T00:00:00Z"
install_mode = "userland"
install_dir = "/fake/dir"
files = []
symlinks = []
registry_source = "file:/fake/test.repo"
allow_insecure = false
"#;
    std::fs::write(&installed_db_path, old_package_toml).unwrap();

    // Verify the package was created with schema_version "0.0"
    let content = std::fs::read_to_string(&installed_db_path).unwrap();
    assert!(content.contains("schema_version = \"0.0\""));

    // The list command should trigger migration when loading the database
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.env("ORA_CONFIG_DIR", env.config_dir())
        .env("ORA_CACHE_DIR", env.cache_dir())
        .env("ORA_DATA_DIR", env.data_dir())
        .arg("list");

    let _ = cmd.assert();

    // Read the database again - it should have been migrated to 0.1
    let content_after = std::fs::read_to_string(&installed_db_path).unwrap();

    // After migration, schema_version should be updated to "0.1"
    assert!(
        content_after.contains("schema_version = \"0.1\""),
        "Schema version should be migrated to 0.1 after loading database. Content:\n{}",
        content_after
    );

    env.cleanup();
}
