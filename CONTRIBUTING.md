Contributing to rWifi

We are thrilled that you want to help improve rWifi! Please follow these guidelines to make sure your contribution matches the style and quality standards of the project.

Developer Environment Setup

To build and test rWifi locally:
Make sure you have the standard Rust toolchain installed on Windows.
Clone this repository.
Check code formatting using cargo fmt check.
Run standard compiler lints using cargo clippy.
Test the debug build using cargo run.
Build and package the final release using cargo build release.

Pull Request Process

Fork the repository and create a new feature branch.
Write clean code and keep your changes focused.
Make sure all compile checks and lints pass.
Document any new features in the README.md or corresponding help manuals.
Open a Pull Request detailing the purpose of your change and any design decisions you made.

TUI Design Principles

If you are modifying the user interface, please keep in mind:
Aesthetics: We use high-contrast RGB tailored color themes matching the Windows Accent Theme. Do not use plain primaries.
Modular architecture: Keep modules focused (e.g. logger, win32, reg, input, worker).
Win32 Integrations: All Win32 platform operations belong in win32.rs wrapped in safe Rust abstractions.
