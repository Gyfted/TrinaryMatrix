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

#[cfg(unix)]
#[test]
fn non_unicode_arguments_fail_without_panicking() {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;

    let invalid = OsString::from_vec(vec![0xff]);
    for args in [
        vec![invalid.clone()],
        vec![OsString::from("--check"), invalid],
    ] {
        let result = Command::new(env!("CARGO_BIN_EXE_trinarymatrix"))
            .args(args)
            .output()
            .unwrap();
        assert_eq!(result.status.code(), Some(1));
        assert!(result.stdout.is_empty());
        assert_eq!(
            String::from_utf8(result.stderr).unwrap(),
            "error: usage: trinarymatrix [--check]\n"
        );
    }
}
