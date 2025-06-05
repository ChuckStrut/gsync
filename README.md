# gsync

An open-source Google Drive client written in Rust.

## Overview

`gsync` aims to be a privacy-friendly Google Drive synchronization tool. The project is divided into three phases:

1. **Rclone Wrapper MVP** – Provide a simple CLI that wraps `rclone` for syncing, mounting and status checks.
2. **Intermediate Transition** – Gradually replace rclone with native Rust code for authentication and basic file operations.
3. **Fully Native Client** – Implement a complete, high-performance sync engine in pure Rust.

## Getting Started

Ensure [`rclone`](https://rclone.org/) is installed and accessible in your `PATH`.

```bash
# Clone the repository
git clone <repo-url>
cd gsync

# Run the setup wizard
cargo run -- init
```

## License

This project is licensed under the [MIT](LICENSE) license.
