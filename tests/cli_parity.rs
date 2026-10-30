use std::process::Command;

#[test]
fn no_wrap_requires_stdin_input_like_upstream() {
    let exe = std::env::var("CARGO_BIN_EXE_cowsay-rust").expect("cargo binary path");
    let output = Command::new(exe).args(["-n", "hello"]).output().expect("binary should run");

    assert!(!output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage:"));
}
