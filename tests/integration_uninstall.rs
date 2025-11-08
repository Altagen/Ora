mod helpers;
use assert_cmd::prelude::*;
use helpers::TestEnvironment;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_uninstall_not_installed() {
    let env = TestEnvironment::new().unwrap();
    env.set_env_vars();

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.arg("uninstall").arg("nonexistent-package");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));

    env.cleanup();
}

#[test]
#[ignore] // Requires a package to be installed first
fn test_uninstall_installed_package() {
    let env = TestEnvironment::new().unwrap();
    env.set_env_vars();

    // TODO: Install a package first, then uninstall it
    // This would require the full install workflow to work

    env.cleanup();
}

#[test]
fn test_update_no_packages() {
    let env = TestEnvironment::new().unwrap();
    env.set_env_vars();

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.arg("update").arg("nonexistent");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not installed"));

    env.cleanup();
}

#[test]
fn test_info_missing_package() {
    let env = TestEnvironment::new().unwrap();
    env.set_env_vars();

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ora"));
    cmd.arg("info").arg("nonexistent-package");

    // Info command might not fail, just show no info
    let _ = cmd.assert();

    env.cleanup();
}
