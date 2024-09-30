use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

fn main() {
    let output = Command::new("git").args(["describe", "--always"]).output().unwrap();
    let git_commit = String::from_utf8(output.stdout).unwrap();
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("assets/commit")
        .ok()
        .map(|mut v| v.write_all(git_commit.as_bytes()).expect("无法写入文件"))
        .expect("无法打开文件");
    println!("cargo:rustc-env=GIT_COMMIT={}", git_commit);
}
