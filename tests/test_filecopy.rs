use std::process::Command;

#[test]
fn test_filecopy() {
    assert!(Command::new("tests/test_filecopy.sh")
                    .output()
                    .unwrap().status.success());
}
