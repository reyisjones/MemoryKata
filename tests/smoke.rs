use assert_cmd::Command;

#[test]
fn binary_runs() {
    let mut cmd = Command::cargo_bin("memorykata").expect("binary built");
    cmd.assert().success();
}
