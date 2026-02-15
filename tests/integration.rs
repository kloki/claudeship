use std::process::{Command, Stdio};

#[test]
fn happy_flow() {
    let input = include_str!("input.json");

    let mut child = Command::new(env!("CARGO_BIN_EXE_claudeship"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    std::io::Write::write_all(&mut child.stdin.take().unwrap(), input.as_bytes()).unwrap();

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    assert!(!output.stdout.is_empty());
}
