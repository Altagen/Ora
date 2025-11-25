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
