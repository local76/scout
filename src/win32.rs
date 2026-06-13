//! Platform abstraction and Win32-specific console integration wrapper.
//!
//! **Taxonomy Classification**: Platform (Win32 Console Integration).

pub use crate::clipboard::copy_text_to_clipboard;
pub use crate::backend::event_log::log_system_event as log_windows_event;
pub use crate::backend::notification::show_toast_notification;
pub use crate::backend::sys_info::{
    query_dark_mode, query_os_version, query_power_status,
    get_dwm_accent_color, GlyphMap,
};
pub use crate::backend::window::{
    query_cursor_pos, get_window_rect, set_window_pos,
    hide_console_at_startup,
};
pub use crate::win32_relaunch::relaunch_in_conhost_if_needed;

pub use crate::backend::wlan::*;
