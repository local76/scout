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
    center_console_window, query_cursor_pos, get_window_rect, set_window_pos,
    BorderlessConsole, ConsoleTitleGuard, SingleInstanceGuard, relaunch_in_conhost_if_needed,
};
pub use crate::wlan::*;

/// Hide the console window early at startup (common pattern for TUI apps).
/// Returns the hwnd if successful (for potential later restore).
#[cfg(windows)]
pub fn hide_console_at_startup() -> Option<*mut std::ffi::c_void> {
    unsafe extern "system" {
        fn GetConsoleWindow() -> *mut std::ffi::c_void;
        fn ShowWindow(hWnd: *mut std::ffi::c_void, nCmdShow: i32) -> i32;
    }
    unsafe {
        let h = GetConsoleWindow();
        if !h.is_null() {
            ShowWindow(h, 0); // SW_HIDE = 0
            Some(h)
        } else {
            None
        }
    }
}

/// Stub implementation for non-Windows platforms.
#[cfg(not(windows))]
pub fn hide_console_at_startup() -> Option<*mut std::ffi::c_void> {
    None
}
