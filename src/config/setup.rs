use dialoguer::Input;

use crate::core::rclone;

pub fn init_wizard() {
    println!("Starting rclone configuration wizard...");

    let remote_name: String = Input::new()
        .with_prompt("Enter a name for your remote")
        .interact_text()
        .unwrap_or_else(|_| "drive".into());

    let _ = rclone::run_rclone(&["config", "create", remote_name.as_str(), "drive"]);
}
