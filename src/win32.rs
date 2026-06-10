#![allow(deprecated)]
//! Platform abstraction and Win32-specific console integration wrapper.
//!
//! **Taxonomy Classification**: Platform (Win32 Console Integration).

pub use library::clipboard::copy_text_to_clipboard;
pub use library::event_log::log_system_event as log_windows_event;
pub use library::notification::show_toast_notification;
pub use library::sys_info::{
    query_dark_mode, query_os_version, query_power_status,
    get_dwm_accent_color, GlyphMap,
};
pub use library::window::{
    query_cursor_pos, get_window_rect, set_window_pos, relaunch_in_conhost_if_needed,
};
pub use crate::backend::wlan::*;

pub use library::hide_console_at_startup;
