use std::process::Command;

pub fn run_rclone(args: &[&str]) -> anyhow::Result<()> {
    Command::new("rclone")
        .args(args)
        .status()
        .map(|_| ())
        .map_err(|e| e.into())
}
