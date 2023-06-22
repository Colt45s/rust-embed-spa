use std::process::Command;

fn main() {
    Command::new("yarn")
        .current_dir("app")
        .arg("run")
        .arg("build")
        .status()
        .expect("failed to build app");
}