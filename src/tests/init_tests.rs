mod utils;
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

/// 'bran init' should initialize a new Git repository. Since the repository already exists
/// in the root of the project, we should see a message indicating that the repository
/// was reinitialized.
#[test]
fn test_init() {
    // Create a temporary directory to run the test. The directory does not contain a Git repository.
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    // Create a ResetWorkingDir instance to reset the working directory after the test.
    let owd = std::env::current_dir().unwrap();
    let mut _reset_wd = utils::ResetWorkingDir::new(owd);

    // Change the working directory to the temporary directory.
    std::env::set_current_dir(&temp_dir).expect("Could not set working directory");

    // Run the 'bran init' command. The directory does not contain a Git repository, so it is initialized.
    Command::cargo_bin("bran")
        .unwrap()
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Initialized empty Git repository"));

    // The workind directory now is reset, and the repository is reinitialized.
    Command::cargo_bin("bran")
        .unwrap()
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Reinitialized existing Git repository in",
        ));
}
