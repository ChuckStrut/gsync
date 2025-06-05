use super::rclone;

pub fn sync() {
    let _ = rclone::run_rclone(&["sync"]);
}

pub fn mount() {
    let _ = rclone::run_rclone(&["mount"]);
}

pub fn status() {
    println!("Status feature not implemented yet");
}
