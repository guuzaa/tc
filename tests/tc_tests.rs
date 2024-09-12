use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_empty_input() {
    let mut cmd = Command::cargo_bin("tc").unwrap();
    cmd.write_stdin("")
        .assert()
        .success()
        .stdout("       0       0       0       0\n");
}

#[test]
fn test_single_word() {
    let mut cmd = Command::cargo_bin("tc").unwrap();
    cmd.write_stdin("hello")
        .assert()
        .success()
        .stdout("       1       1       5       1\n");
}

#[test]
fn test_multiple_words() {
    let mut cmd = Command::cargo_bin("tc").unwrap();
    cmd.write_stdin("hello world\nrust is great")
        .assert()
        .success()
        .stdout("       2       5      25       6\n");
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
        .stdout(predicate::str::contains("       2       6      30      11"));
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
        .stdout(predicate::str::contains("       1       2      14       5"))
        .stdout(predicate::str::contains("       1       4      22       6"))
        .stdout(predicate::str::contains(
            "       2       6      36      11 total",
        ));
}

#[test]
fn test_non_existent_file() {
    let mut cmd = Command::cargo_bin("tc").unwrap();
    cmd.arg("non_existent_file.txt")
        .assert()
        .success()
        .stderr(predicate::str::contains("Error opening file"));
}

#[test]
fn test_specific_options() {
    let mut cmd = Command::cargo_bin("tc").unwrap();
    cmd.args(&["-lw"])
        .write_stdin("Hello\nWorld\nRust")
        .assert()
        .success()
        .stdout("       3       3\n");
}

#[test]
fn test_utf8_input() {
    let mut cmd = Command::cargo_bin("tc").unwrap();
    cmd.write_stdin("Hello, 世界!")
        .assert()
        .success()
        .stdout("       1       2      10       8\n");
}

#[test]
fn test_different_tokenizer_model() {
    let mut cmd = Command::cargo_bin("tc").unwrap();
    cmd.args(&["--model", "gpt4o"])
        .write_stdin("Hello, world!")
        .assert()
        .success()
        .stdout("       1       2      13       4\n");
}
