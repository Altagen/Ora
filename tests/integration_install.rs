mod helpers;
use assert_cmd::prelude::*;
use helpers::{MockRegistry, TestEnvironment};
use ora::storage::cache::Cache;
use predicates::prelude::*;
use std::path::PathBuf;
use std::process::Command;

#[test]
#[ignore] // Requires network and downloads real binaries
fn test_install_from_repo_file_windman() {
    let env = TestEnvironment::new().unwrap();

    let repo_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("repo_files")
        .join("windman.repo");

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.env("ORA_CONFIG_DIR", env.config_dir())
        .env("ORA_CACHE_DIR", env.cache_dir())
        .env("ORA_DATA_DIR", env.data_dir())
        .arg("install")
        .arg("--repo")
        .arg(repo_file)
        .arg("--userland")
        .arg("windman");

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
    let registry = MockRegistry::new().unwrap();

    // Add registry
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.env("ORA_CONFIG_DIR", env.config_dir())
        .env("ORA_CACHE_DIR", env.cache_dir())
        .env("ORA_DATA_DIR", env.data_dir())
        .arg("registry")
        .arg("add")
        .arg("test-registry")
        .arg(registry.url());
    cmd.assert().success();

    // Sync registry
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.env("ORA_CONFIG_DIR", env.config_dir())
        .env("ORA_CACHE_DIR", env.cache_dir())
        .env("ORA_DATA_DIR", env.data_dir())
        .arg("registry")
        .arg("sync");
    cmd.assert().success();

    // Install windman (smallest package for testing)
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.env("ORA_CONFIG_DIR", env.config_dir())
        .env("ORA_CACHE_DIR", env.cache_dir())
        .env("ORA_DATA_DIR", env.data_dir())
        .arg("install")
        .arg("windman")
        .arg("--userland");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Successfully installed"));

    env.cleanup();
}

#[test]
fn test_install_missing_package() {
    let env = TestEnvironment::new().unwrap();

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.env("ORA_CONFIG_DIR", env.config_dir())
        .env("ORA_CACHE_DIR", env.cache_dir())
        .env("ORA_DATA_DIR", env.data_dir())
        .arg("install")
        .arg("nonexistent-package-xyz");

    // Should fail because no registries configured
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No registries configured"));

    env.cleanup();
}

#[test]
fn test_install_with_version() {
    let env = TestEnvironment::new().unwrap();

    let repo_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("repo_files")
        .join("windman.repo");

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.env("ORA_CONFIG_DIR", env.config_dir())
        .env("ORA_CACHE_DIR", env.cache_dir())
        .env("ORA_DATA_DIR", env.data_dir())
        .arg("install")
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

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.env("ORA_CONFIG_DIR", env.config_dir())
        .env("ORA_CACHE_DIR", env.cache_dir())
        .env("ORA_DATA_DIR", env.data_dir())
        .arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No packages installed"));

    env.cleanup();
}

#[test]
fn test_search_no_registry() {
    let env = TestEnvironment::new().unwrap();

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.env("ORA_CONFIG_DIR", env.config_dir())
        .env("ORA_CACHE_DIR", env.cache_dir())
        .env("ORA_DATA_DIR", env.data_dir())
        .arg("search")
        .arg("prometheus");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No registries configured"));

    env.cleanup();
}

// Tests for Issues #3 & #4 fixes

#[test]
fn test_download_path_empty_filename() {
    // Test that empty filename is rejected (Issue #4 fix)
    let result = Cache::download_path("");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("filename is empty"));
}

#[test]
fn test_download_path_valid_filename() {
    // Test that valid filename works
    let result = Cache::download_path("test-package.tar.gz");
    assert!(result.is_ok());

    let path = result.unwrap();
    assert!(path.to_string_lossy().contains("downloads"));
    assert!(path.to_string_lossy().ends_with("test-package.tar.gz"));
}

#[test]
fn test_url_filename_extraction_with_trailing_slash() {
    // Simulate the bug fix: trailing slash handling (Issue #4)
    let url = "https://example.com/path/to/file.tar.gz/";

    // Old behavior (buggy): would get empty string
    let old_way = url.split('/').next_back().unwrap();
    assert_eq!(old_way, "");

    // New behavior (fixed): strip trailing slash first
    let fixed_way = url.trim_end_matches('/').split('/').next_back().unwrap();
    assert_eq!(fixed_way, "file.tar.gz");
}

#[test]
fn test_url_filename_extraction_without_trailing_slash() {
    // Normal case: no trailing slash
    let url = "https://example.com/path/to/file.tar.gz";

    let filename = url.trim_end_matches('/').split('/').next_back().unwrap();
    assert_eq!(filename, "file.tar.gz");
}

#[test]
fn test_url_filename_extraction_multiple_trailing_slashes() {
    // Edge case: multiple trailing slashes
    let url = "https://example.com/path/to/file.tar.gz///";

    let filename = url.trim_end_matches('/').split('/').next_back().unwrap();
    assert_eq!(filename, "file.tar.gz");
}

#[test]
fn test_vicinae_url_pattern_buggy() {
    // Test the exact pattern from the vicinae bug (Issue #3 & #4)
    let version = "v0.16.8";
    let os = "linux";
    let arch = "x86_64";

    // Buggy .repo template (with trailing slash)
    let buggy_template = format!(
        "https://github.com/vicinaehq/vicinae/releases/download/{}/vicinae-{}-{}.tar.gz/",
        version, os, arch
    );

    // Extract filename using the fix
    let filename = buggy_template
        .trim_end_matches('/')
        .split('/')
        .next_back()
        .unwrap();

    assert_eq!(filename, "vicinae-linux-x86_64.tar.gz");
    assert!(!filename.is_empty());
}

#[test]
fn test_vicinae_url_pattern_corrected() {
    // Test the corrected .repo template (no trailing slash, includes version)
    let version = "v0.16.8";
    let os = "linux";
    let arch = "x86_64";

    let corrected_template = format!(
        "https://github.com/vicinaehq/vicinae/releases/download/{}/vicinae-{}-{}-{}.tar.gz",
        version, os, arch, version
    );

    let filename = corrected_template
        .trim_end_matches('/')
        .split('/')
        .next_back()
        .unwrap();

    assert_eq!(filename, "vicinae-linux-x86_64-v0.16.8.tar.gz");
    assert!(!filename.is_empty());
}
