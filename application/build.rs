use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

const COMMIT_FILE: &str = "assets/commit";

fn main() {
    let output = Command::new("git").args(["describe", "--always"]).output().unwrap();
    let git_commit = String::from_utf8(output.stdout).unwrap();
    // write git commit sha into file "assets/commit"
    OpenOptions::new().write(true).truncate(true).create(true)
        .open(COMMIT_FILE).expect("can not open assets/commit file")
        .write_all(git_commit.as_bytes()).expect("can not write assets/commit file");
}
