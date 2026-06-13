//! Backend abstraction layer for Wi-Fi scanner and connections.
//!
//! **Taxonomy Classification**: Platform Abstraction.

pub mod wlan;
pub mod config;
pub mod identity;
pub mod monitors;
pub mod shell_terminal;
pub mod sys_info;
pub mod window;
pub mod event_log;
pub mod notification;

pub mod sysinfo_shim;
