use std::process::Command;

#[test]
fn check_demo_succeeds() {
    let result = Command::new(env!("CARGO_BIN_EXE_trinarymatrix"))
        .arg("--check")
        .output()
        .unwrap();
    assert!(result.status.success());
    let stdout = String::from_utf8(result.stdout).unwrap();
    assert!(stdout.contains("add/sub/skip output: [1, -3]"));
    assert!(stdout.contains("check: passed"));
}

#[test]
fn invalid_arguments_fail() {
    let result = Command::new(env!("CARGO_BIN_EXE_trinarymatrix"))
        .arg("--unknown")
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8(result.stderr).unwrap().contains("usage:"));
}
