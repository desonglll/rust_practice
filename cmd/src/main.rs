use std::process::Command;
fn main() {
    let mut cmd = Command::new("ls");
    let output = cmd.output().expect("failed to execute process");
}
