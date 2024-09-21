use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::{self, File};
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_empty_input() {
    let mut cmd = Command::cargo_bin("tc").unwrap();
    cmd.write_stdin("")
        .assert()
        .success()
        .stdout("       0        0        0        0\n");
}

#[test]
fn test_single_word() {
    let mut cmd = Command::cargo_bin("tc").unwrap();
    cmd.write_stdin("hello")
        .assert()
        .success()
        .stdout("       1        1        5        1\n");
}

#[test]
fn test_multiple_words() {
    let mut cmd = Command::cargo_bin("tc").unwrap();
    cmd.write_stdin("hello world\nrust is great")
        .assert()
        .success()
        .stdout("       2        5       25        6\n");
}

#[test]
fn test_file_input() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "Hello, world!").unwrap();
    writeln!(file, "This is a test.").unwrap();

    let mut cmd = Command::cargo_bin("tc").unwrap();
    cmd.arg(file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "       2        6       30       11",
        ));
}

#[test]
fn test_multiple_files() {
    let dir = tempdir().unwrap();
    let file1_path = dir.path().join("test1.txt");
    let file2_path = dir.path().join("test2.txt");

    let mut file1 = File::create(&file1_path).unwrap();
    writeln!(file1, "Hello, world!").unwrap();

    let mut file2 = File::create(&file2_path).unwrap();
    writeln!(file2, "This is another test.").unwrap();

    let mut cmd = Command::cargo_bin("tc").unwrap();
    cmd.arg(&file1_path)
        .arg(&file2_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "       1        2       14        5",
        ))
        .stdout(predicate::str::contains(
            "       1        4       22        6",
        ))
        .stdout(predicate::str::contains(
            "       2        6       36       11 total",
        ));
    fs::remove_file(file1_path).unwrap();
    fs::remove_file(file2_path).unwrap();
}

#[test]
fn test_non_existent_file() {
    let mut cmd = Command::cargo_bin("tc").unwrap();
    cmd.arg("non_existent_file.txt")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "tc: non_existent_file.txt: No such file",
        ));
}

#[test]
fn test_specific_options() {
    let mut cmd = Command::cargo_bin("tc").unwrap();
    cmd.args(&["-lw"])
        .write_stdin("Hello\nWorld\nRust")
        .assert()
        .success()
        .stdout("       3        3\n");
}

#[test]
fn test_utf8_input() {
    let mut cmd = Command::cargo_bin("tc").unwrap();
    cmd.write_stdin("Hello, 世界!")
        .assert()
        .success()
        .stdout("       1        2       10        8\n");
}

#[test]
fn test_different_tokenizer_model() {
    let mut cmd = Command::cargo_bin("tc").unwrap();
    cmd.args(&["--model", "gpt4o"])
        .write_stdin("Hello, world!")
        .assert()
        .success()
        .stdout("       1        2       13        4\n");
}

#[test]
fn test_error_code_without_termination() {
    let dir = tempdir().unwrap();
    let existing_file1 = dir.path().join("existing1.txt");
    let existing_file2 = dir.path().join("existing2.txt");
    let non_existent_file = dir.path().join("non_existent.txt");

    // Create existing files
    let mut file1 = File::create(&existing_file1).unwrap();
    writeln!(file1, "This is an existing file.").unwrap();

    let mut file2 = File::create(&existing_file2).unwrap();
    writeln!(file2, "This is another existing file.").unwrap();

    let mut cmd = Command::cargo_bin("tc").unwrap();
    cmd.arg(&existing_file1)
        .arg(&non_existent_file)
        .arg(&existing_file2)
        .assert()
        .failure()
        .code(1) // Expect an error code of 1
        .stdout(predicate::str::contains(
            "       1        5       26        7",
        )) // Output for first existing file
        .stderr(predicate::str::contains("No such file")) // Error message for non-existent file
        .stdout(predicate::str::contains(
            "       1        5       31        7",
        )) // Output for second existing file
        .stdout(predicate::str::contains(
            "       2       10       57       14 total",
        )); // Total count

    // Verify that the process didn't terminate early by checking if it processed both existing files
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("existing1.txt"));
    assert!(stdout.contains("existing2.txt"));
    assert!(stdout.contains("total"));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("non_existent.txt"));
    fs::remove_file(existing_file1).unwrap();
    fs::remove_file(existing_file2).unwrap();
}

#[cfg(unix)]
#[test]
fn test_permission_denied() {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("no_permission.txt");

    let mut file = File::create(&file_path).unwrap();
    write!(file, "This is a test file.").unwrap();

    let mut perms = fs::metadata(&file_path).unwrap().permissions();
    perms.set_mode(0o000);
    fs::set_permissions(&file_path, perms).unwrap();

    let mut cmd = Command::cargo_bin("tc").unwrap();
    cmd.arg(&file_path)
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Permission denied"));

    let mut perms = fs::metadata(&file_path).unwrap().permissions();
    perms.set_mode(0o644);
    fs::set_permissions(&file_path, perms).unwrap();
    let mut cmd = Command::cargo_bin("tc").unwrap();
    cmd.arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "       1        5       20        6",
        ));
    fs::remove_file(&file_path).unwrap();
}

#[cfg(unix)]
#[test]
fn test_directory_counting() {
    let dir = tempdir().unwrap();

    // Create a subdirectory
    let subdir_path = dir.path().join("subdir");
    fs::create_dir(&subdir_path).unwrap();

    let mut cmd = Command::cargo_bin("tc").unwrap();
    cmd.arg(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains(format!(
            "tc: {}: Error reading file:",
            dir.path().to_string_lossy()
        )));
    fs::remove_dir_all(dir).unwrap();
}
