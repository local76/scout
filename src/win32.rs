#![allow(dead_code)]
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
#[allow(non_snake_case)]
pub struct MONITORINFO {
    pub cbSize: u32,
    pub rcMonitor: RECT,
    pub rcWork: RECT,
    pub dwFlags: u32,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct COORD {
    pub x: i16,
    pub y: i16,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct SMALL_RECT {
    pub left: i16,
    pub top: i16,
    pub right: i16,
    pub bottom: i16,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
#[allow(non_snake_case)]
pub struct CONSOLE_SELECTION_INFO {
    pub dwFlags: u32,
    pub dwSelectionAnchor: COORD,
    pub srSelection: SMALL_RECT,
}

#[cfg(windows)]
#[link(name = "user32")]
unsafe extern "system" {
    fn GetSystemMetrics(n_index: i32) -> i32;
    fn GetWindowRect(h_wnd: *mut std::ffi::c_void, lp_rect: *mut RECT) -> i32;
    fn SetWindowLongPtrW(h_wnd: *mut std::ffi::c_void, n_index: i32, dw_new_long: isize) -> isize;
    fn GetWindowLongPtrW(h_wnd: *mut std::ffi::c_void, n_index: i32) -> isize;
    fn SetWindowPos(
        h_wnd: *mut std::ffi::c_void,
        h_wnd_insert_after: *mut std::ffi::c_void,
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        u_flags: u32,
    ) -> i32;
    fn GetDpiForWindow(h_wnd: *mut std::ffi::c_void) -> u32;
    fn OpenClipboard(h_wnd_new_owner: *mut std::ffi::c_void) -> i32;
    fn EmptyClipboard() -> i32;
    fn SetClipboardData(u_format: u32, h_mem: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn CloseClipboard() -> i32;
    fn MonitorFromWindow(h_wnd: *mut std::ffi::c_void, dw_flags: u32) -> *mut std::ffi::c_void;
    fn GetMonitorInfoW(h_monitor: *mut std::ffi::c_void, lp_mi: *mut MONITORINFO) -> i32;
    fn GetAsyncKeyState(v_key: i32) -> i16;
}

#[cfg(windows)]
#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetConsoleWindow() -> *mut std::ffi::c_void;
    fn CreateMutexW(
        lp_mutex_attributes: *const std::ffi::c_void,
        b_initial_owner: i32,
        lp_name: *const u16,
    ) -> *mut std::ffi::c_void;
    fn GetLastError() -> u32;
    fn CloseHandle(h_object: *mut std::ffi::c_void) -> i32;
    fn GetConsoleTitleW(lp_console_title: *mut u16, n_size: u32) -> u32;
    fn SetConsoleTitleW(lp_console_title: *const u16) -> i32;
    fn GetLogicalDriveStringsW(n_buffer_length: u32, lp_buffer: *mut u16) -> u32;
    fn GetDiskFreeSpaceExW(
        lp_directory_name: *const u16,
        lp_free_bytes_available_to_caller: *mut u64,
        lp_total_number_of_bytes: *mut u64,
        lp_total_number_of_free_bytes: *mut u64,
    ) -> i32;
    fn GlobalAlloc(u_flags: u32, dw_bytes: usize) -> *mut std::ffi::c_void;
    fn GlobalLock(h_mem: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn GlobalUnlock(h_mem: *mut std::ffi::c_void) -> i32;
    fn GlobalFree(h_mem: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn GetStdHandle(n_std_handle: u32) -> *mut std::ffi::c_void;
    fn ReadConsoleOutputCharacterW(
        h_console_output: *mut std::ffi::c_void,
        lp_character: *mut u16,
        n_length: u32,
        dw_read_coord: COORD,
        lp_number_of_chars_read: *mut u32,
    ) -> i32;
    fn GetConsoleSelectionInfo(lp_console_selection_info: *mut CONSOLE_SELECTION_INFO) -> i32;
    fn CreateFileW(
        lp_file_name: *const u16,
        dw_desired_access: u32,
        dw_share_mode: u32,
        lp_security_attributes: *const std::ffi::c_void,
        dw_creation_disposition: u32,
        dw_flags_and_attributes: u32,
        h_template_file: *mut std::ffi::c_void,
    ) -> *mut std::ffi::c_void;
}

#[cfg(windows)]
#[link(name = "dwmapi")]
unsafe extern "system" {
    fn DwmGetColorizationColor(pcr_color: *mut u32, pf_opaque_blend: *mut i32) -> i32;
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
#[allow(non_snake_case)]
pub struct SERVICE_STATUS {
    pub dwServiceType: u32,
    pub dwCurrentState: u32,
    pub dwControlsAccepted: u32,
    pub dwWin32ExitCode: u32,
    pub dwServiceSpecificExitCode: u32,
    pub dwCheckPoint: u32,
    pub dwWaitHint: u32,
}

#[cfg(windows)]
#[link(name = "advapi32")]
unsafe extern "system" {
    fn RegisterEventSourceW(
        lp_unc_server_name: *const u16,
        lp_source_name: *const u16,
    ) -> *mut std::ffi::c_void;

    fn ReportEventW(
        h_event_log: *mut std::ffi::c_void,
        w_type: u16,
        w_category: u16,
        dw_event_id: u32,
        lp_user_sid: *mut std::ffi::c_void,
        w_num_strings: u16,
        dw_data_size: u32,
        lp_strings: *const *const u16,
        lp_raw_data: *mut std::ffi::c_void,
    ) -> i32;

    fn DeregisterEventSource(h_event_log: *mut std::ffi::c_void) -> i32;

    fn OpenSCManagerW(
        lp_machine_name: *const u16,
        lp_database_name: *const u16,
        dw_desired_access: u32,
    ) -> *mut std::ffi::c_void;

    fn OpenServiceW(
        h_sc_manager: *mut std::ffi::c_void,
        lp_service_name: *const u16,
        dw_desired_access: u32,
    ) -> *mut std::ffi::c_void;

    fn QueryServiceStatus(
        h_service: *mut std::ffi::c_void,
        lp_service_status: *mut SERVICE_STATUS,
    ) -> i32;

    fn CloseServiceHandle(h_sc_object: *mut std::ffi::c_void) -> i32;
}

/// Bounding rect of the console window.
#[allow(dead_code)]
pub fn get_console_rect() -> Option<RECT> {
    #[cfg(windows)]
    {
        let hwnd = unsafe { GetConsoleWindow() };
        if hwnd.is_null() {
            return None;
        }
        let mut r = RECT::default();
        if unsafe { GetWindowRect(hwnd, &mut r) } != 0 {
            Some(r)
        } else {
            None
        }
    }
    #[cfg(not(windows))]
    None
}

/// Ensures only one instance of the TUI application is active at any time.
pub struct SingleInstanceGuard {
    #[allow(dead_code)]
    handle: *mut std::ffi::c_void,
}

impl SingleInstanceGuard {
    pub fn try_new() -> Result<Self, String> {
        #[cfg(windows)]
        {
            let name: Vec<u16> = "Local\\rwif_SingleInstanceMutex_2026\0"
                .encode_utf16()
                .collect();
            let handle = unsafe { CreateMutexW(std::ptr::null(), 1, name.as_ptr()) };
            if handle.is_null() {
                return Err("Failed to create single-instance mutex.".to_string());
            }

            let err = unsafe { GetLastError() };
            if err == 183 {
                // ERROR_ALREADY_EXISTS = 183
                unsafe { CloseHandle(handle) };

                // Try to find and terminate other rwif.exe processes to resolve zombie instances
                use sysinfo::System;
                let mut sys = System::new_all();
                sys.refresh_all();
                if let Ok(current_pid) = sysinfo::get_current_pid() {
                    let mut terminated_any = false;
                    for (&pid, process) in sys.processes() {
                        if pid != current_pid {
                            let p_name = process.name().to_lowercase();
                            if p_name == "rwif.exe" || p_name == "rwif" {
                                let _ = process.kill();
                                terminated_any = true;
                            }
                        }
                    }
                    if terminated_any {
                        // Settle time for OS to release mutex
                        std::thread::sleep(std::time::Duration::from_millis(100));
                        // Retry acquiring mutex
                        let new_handle = unsafe { CreateMutexW(std::ptr::null(), 1, name.as_ptr()) };
                        if !new_handle.is_null() {
                            let new_err = unsafe { GetLastError() };
                            if new_err != 183 {
                                return Ok(SingleInstanceGuard { handle: new_handle });
                            }
                            unsafe { CloseHandle(new_handle) };
                        }
                    }
                }

                return Err("Another instance of this application is already running.".to_string());
            }

            Ok(SingleInstanceGuard { handle })
        }
        #[cfg(not(windows))]
        {
            Ok(SingleInstanceGuard {
                handle: std::ptr::null_mut(),
            })
        }
    }
}

impl Drop for SingleInstanceGuard {
    fn drop(&mut self) {
        #[cfg(windows)]
        if !self.handle.is_null() {
            unsafe { CloseHandle(self.handle) };
        }
    }
}

/// Strips standard console headers/borders and centers window dynamically (DPI-aware).
pub struct BorderlessConsole {
    hwnd: *mut std::ffi::c_void,
    original_style: isize,
    original_rect: RECT,
    active: bool,
}

impl BorderlessConsole {
    pub fn enable() -> Self {
        #[cfg(windows)]
        {
            let hwnd = unsafe { GetConsoleWindow() };
            if hwnd.is_null() {
                return BorderlessConsole {
                    hwnd: std::ptr::null_mut(),
                    original_style: 0,
                    original_rect: RECT::default(),
                    active: false,
                };
            }

            let original_style = unsafe { GetWindowLongPtrW(hwnd, -16) }; // GWL_STYLE = -16
            let mut original_rect = RECT::default();
            unsafe {
                GetWindowRect(hwnd, &mut original_rect);
            }

            // Strip border decorations: WS_CAPTION | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX | WS_SYSMENU
            let style_mask = 0x00C00000 | 0x00040000 | 0x00020000 | 0x00010000 | 0x00080000;
            let new_style = original_style & !(style_mask as isize);
            unsafe {
                SetWindowLongPtrW(hwnd, -16, new_style);
            }

            let dpi = unsafe { GetDpiForWindow(hwnd) };
            let scale = dpi as f32 / 96.0;
            let width = (780.0 * scale) as i32;
            let height = (520.0 * scale) as i32;

            let mut x = 100;
            let mut y = 100;
            let h_monitor = unsafe { MonitorFromWindow(hwnd, 2) }; // MONITOR_DEFAULTTONEAREST = 2
            if !h_monitor.is_null() {
                let mut mi = MONITORINFO::default();
                mi.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
                if unsafe { GetMonitorInfoW(h_monitor, &mut mi) } != 0 {
                    let monitor_w = mi.rcWork.right - mi.rcWork.left;
                    let monitor_h = mi.rcWork.bottom - mi.rcWork.top;
                    x = mi.rcWork.left + (monitor_w - width) / 2;
                    y = mi.rcWork.top + (monitor_h - height) / 2;
                }
            }

            unsafe {
                SetWindowPos(
                    hwnd,
                    std::ptr::null_mut(),
                    x,
                    y,
                    width,
                    height,
                    0x0020 | 0x0004 | 0x0010, // SWP_FRAMECHANGED | SWP_NOZORDER | SWP_NOACTIVATE
                );
            }

            BorderlessConsole {
                hwnd,
                original_style,
                original_rect,
                active: true,
            }
        }
        #[cfg(not(windows))]
        {
            BorderlessConsole {
                hwnd: std::ptr::null_mut(),
                original_style: 0,
                original_rect: RECT::default(),
                active: false,
            }
        }
    }
}

impl Drop for BorderlessConsole {
    fn drop(&mut self) {
        #[cfg(windows)]
        if self.active && !self.hwnd.is_null() {
            unsafe {
                SetWindowLongPtrW(self.hwnd, -16, self.original_style);
                let width = self.original_rect.right - self.original_rect.left;
                let height = self.original_rect.bottom - self.original_rect.top;
                SetWindowPos(
                    self.hwnd,
                    std::ptr::null_mut(),
                    self.original_rect.left,
                    self.original_rect.top,
                    width,
                    height,
                    0x0020 | 0x0004 | 0x0010, // SWP_FRAMECHANGED | SWP_NOZORDER | SWP_NOACTIVATE
                );
            }
        }
    }
}

/// Center the console window on the primary display or active monitor.
pub fn center_console_window() {
    #[cfg(windows)]
    unsafe {
        let hwnd = GetConsoleWindow();
        if hwnd.is_null() {
            return;
        }

        let mut rect = RECT::default();
        if GetWindowRect(hwnd, &mut rect) != 0 {
            let width = rect.right - rect.left;
            let height = rect.bottom - rect.top;

            let h_monitor = MonitorFromWindow(hwnd, 2); // MONITOR_DEFAULTTONEAREST = 2
            if !h_monitor.is_null() {
                let mut mi = MONITORINFO::default();
                mi.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
                if GetMonitorInfoW(h_monitor, &mut mi) != 0 {
                    let monitor_w = mi.rcWork.right - mi.rcWork.left;
                    let monitor_h = mi.rcWork.bottom - mi.rcWork.top;

                    let x = mi.rcWork.left + (monitor_w - width) / 2;
                    let y = mi.rcWork.top + (monitor_h - height) / 2;

                    SetWindowPos(
                        hwnd,
                        std::ptr::null_mut(),
                        x,
                        y,
                        width,
                        height,
                        0x0001 | 0x0004 | 0x0010, // SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE
                    );
                }
            }
        }
    }
}

pub struct ConsoleTitleGuard {
    original_title: Option<Vec<u16>>,
}

impl ConsoleTitleGuard {
    pub fn new(new_title: &str) -> Self {
        #[cfg(windows)]
        {
            let mut buf = [0u16; 512];
            let len = unsafe { GetConsoleTitleW(buf.as_mut_ptr(), buf.len() as u32) };
            let original_title = if len > 0 {
                Some(buf[..len as usize].to_vec())
            } else {
                None
            };

            let title_w: Vec<u16> = new_title.encode_utf16().chain(std::iter::once(0)).collect();
            unsafe {
                SetConsoleTitleW(title_w.as_ptr());
            }

            ConsoleTitleGuard { original_title }
        }
        #[cfg(not(windows))]
        {
            ConsoleTitleGuard {
                original_title: None,
            }
        }
    }
}

impl Drop for ConsoleTitleGuard {
    fn drop(&mut self) {
        #[cfg(windows)]
        if let Some(ref title) = self.original_title {
            let mut title_null = title.clone();
            title_null.push(0);
            unsafe {
                SetConsoleTitleW(title_null.as_ptr());
            }
        }
    }
}

/// Retrieve dynamic Windows Accent Color using dwmapi.
pub fn get_dwm_accent_color() -> ratatui::style::Color {
    #[cfg(windows)]
    {
        let mut color: u32 = 0;
        let mut opaque: i32 = 0;
        let hr = unsafe { DwmGetColorizationColor(&mut color, &mut opaque) };
        if hr == 0 {
            // ARGB color (0xAARRGGBB) -> extract RGB
            let r = ((color >> 16) & 0xFF) as u8;
            let g = ((color >> 8) & 0xFF) as u8;
            let b = (color & 0xFF) as u8;
            return ratatui::style::Color::Rgb(r, g, b);
        }
    }
    ratatui::style::Color::Rgb(0, 245, 255)
}

/// Query system metrics for layout sizing
pub fn get_system_screen_resolution() -> (i32, i32) {
    #[cfg(windows)]
    {
        let screen_w = unsafe { GetSystemMetrics(0) }; // SM_CXSCREEN
        let screen_h = unsafe { GetSystemMetrics(1) }; // SM_CYSCREEN
        (screen_w, screen_h)
    }
    #[cfg(not(windows))]
    (1920, 1080)
}

/// Query native console window DPI.
pub fn get_console_window_dpi() -> u32 {
    #[cfg(windows)]
    {
        let hwnd = unsafe { GetConsoleWindow() };
        if !hwnd.is_null() {
            unsafe { GetDpiForWindow(hwnd) }
        } else {
            96
        }
    }
    #[cfg(not(windows))]
    96
}

/// Query the Windows OS version and build number.
pub fn query_os_version() -> String {
    #[cfg(windows)]
    {
        let product_name = crate::reg::read_string(
            crate::reg::HKEY_LOCAL_MACHINE,
            "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion",
            "ProductName",
        )
        .unwrap_or_else(|| "Windows".to_string());
        let current_build = crate::reg::read_string(
            crate::reg::HKEY_LOCAL_MACHINE,
            "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion",
            "CurrentBuild",
        )
        .unwrap_or_default();
        let display_version = crate::reg::read_string(
            crate::reg::HKEY_LOCAL_MACHINE,
            "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion",
            "DisplayVersion",
        )
        .unwrap_or_default();

        let mut final_product = product_name;
        if final_product.starts_with("Windows 10") {
            if let Ok(build) = current_build.parse::<u32>() {
                if build >= 22000 {
                    final_product = final_product.replace("Windows 10", "Windows 11");
                }
            }
        }

        let mut parts = vec![final_product];
        if !display_version.is_empty() {
            parts.push(display_version);
        }
        if !current_build.is_empty() {
            parts.push(format!("(Build {})", current_build));
        }
        parts.join(" ")
    }
    #[cfg(not(windows))]
    {
        "Generic OS".to_string()
    }
}

/// Query dynamic dark mode status.
pub fn query_dark_mode() -> bool {
    crate::reg::read_u32(
        crate::reg::HKEY_CURRENT_USER,
        r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
        "AppsUseLightTheme",
    )
    .map(|v| v == 0)
    .unwrap_or(true)
}

#[derive(Debug, Clone, Default)]
pub struct PowerStatus {
    pub ac_online: bool,
    pub battery_percent: u8,
}

/// Query system battery life and charging source using GetSystemPowerStatus.
pub fn query_power_status() -> Option<PowerStatus> {
    #[cfg(windows)]
    {
        #[repr(C)]
        struct SYSTEM_POWER_STATUS {
            ac_line_status: u8,
            battery_flag: u8,
            battery_life_percent: u8,
            system_status_flag: u8,
            battery_life_time: u32,
            battery_full_life_time: u32,
        }

        #[link(name = "kernel32")]
        unsafe extern "system" {
            fn GetSystemPowerStatus(lp_system_power_status: *mut SYSTEM_POWER_STATUS) -> i32;
        }

        let mut status: SYSTEM_POWER_STATUS = unsafe { std::mem::zeroed() };
        if unsafe { GetSystemPowerStatus(&mut status) } != 0 {
            return Some(PowerStatus {
                ac_online: status.ac_line_status == 1,
                battery_percent: status.battery_life_percent,
            });
        }
    }
    None
}

#[derive(Debug, Clone, Default)]
pub struct SystemBiosInfo {
    pub manufacturer: String,
    pub product: String,
    pub model: String,
}

/// Query BIOS system details from registry.
pub fn query_bios_info() -> Option<SystemBiosInfo> {
    #[cfg(windows)]
    {
        use winreg::RegKey;
        use winreg::enums::HKEY_LOCAL_MACHINE;
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let path = r"HARDWARE\DESCRIPTION\System\BIOS";
        if let Ok(key) = hklm.open_subkey(path) {
            let manufacturer = key
                .get_value::<String, _>("SystemManufacturer")
                .unwrap_or_default();
            let product = key
                .get_value::<String, _>("SystemProductName")
                .unwrap_or_default();
            let model = key
                .get_value::<String, _>("BaseBoardProduct")
                .unwrap_or_default();
            return Some(SystemBiosInfo {
                manufacturer: manufacturer.trim().to_string(),
                product: product.trim().to_string(),
                model: model.trim().to_string(),
            });
        }
    }
    None
}

/// Query process hierarchy to detect active Shell and Terminal Emulator.
pub fn query_shell_and_terminal() -> (String, String) {
    let mut shell = "Unknown Shell".to_string();
    let mut terminal = "Unknown Terminal".to_string();

    #[cfg(windows)]
    {
        use sysinfo::System;
        let mut sys = System::new_all();
        sys.refresh_all();

        let mut current_pid = sysinfo::get_current_pid().ok();
        let mut depth = 0;

        while let Some(pid) = current_pid {
            if depth > 12 {
                break;
            }
            if let Some(process) = sys.process(pid) {
                let name = process.name().to_lowercase();
                if shell == "Unknown Shell" {
                    if name.contains("powershell") || name.contains("pwsh") {
                        shell = "PowerShell".to_string();
                    } else if name == "cmd.exe" || name == "cmd" {
                        shell = "CMD".to_string();
                    } else if name.contains("bash") || name.contains("sh") || name.contains("zsh") {
                        shell = name.replace(".exe", "");
                    }
                }

                if terminal == "Unknown Terminal" {
                    if name.contains("windowsterminal") || name == "openconsole.exe" {
                        terminal = "Windows Terminal".to_string();
                    } else if name.contains("code") {
                        terminal = "VS Code Terminal".to_string();
                    } else if name.contains("alacritty") {
                        terminal = "Alacritty".to_string();
                    } else if name.contains("wezterm") {
                        terminal = "WezTerm".to_string();
                    } else if name.contains("conhost") {
                        terminal = "Windows Console Host".to_string();
                    }
                }

                current_pid = process.parent();
                depth += 1;
            } else {
                break;
            }
        }
    }

    (shell, terminal)
}

#[derive(Debug, Clone, Copy)]
pub struct GlyphMap {
    pub status_ok: &'static str,
    pub status_err: &'static str,
    pub info: &'static str,
    pub warning: &'static str,
    pub cpu: &'static str,
    pub gpu: &'static str,
    pub memory: &'static str,
    pub disk: &'static str,
    pub package: &'static str,
    pub battery: &'static str,
    pub shell: &'static str,
    pub terminal: &'static str,
    pub network: &'static str,
    pub play: &'static str,
    pub play_empty: &'static str,
}

impl GlyphMap {
    pub fn load() -> Self {
        let (_, terminal) = query_shell_and_terminal();
        if terminal == "Windows Console Host" {
            Self {
                status_ok: "[OK]",
                status_err: "[ERR]",
                info: "[i]",
                warning: "[!]",
                cpu: "[CPU]",
                gpu: "[GPU]",
                memory: "[RAM]",
                disk: "[DISK]",
                package: "[PKG]",
                battery: "[BAT]",
                shell: "[SH]",
                terminal: "[TERM]",
                network: "[NET]",
                play: "> ",
                play_empty: "  ",
            }
        } else {
            Self {
                status_ok: "✔️",
                status_err: "❌",
                info: "ℹ️",
                warning: "⚠️",
                cpu: "🧠",
                gpu: "🎮",
                memory: "📟",
                disk: "💾",
                package: "📦",
                battery: "🔋",
                shell: "🐚",
                terminal: "📟",
                network: "🌐",
                play: "▶ ",
                play_empty: "  ",
            }
        }
    }
}

/// Trigger a native Windows Toast Notification using a PowerShell/WinRT shim.
pub fn show_toast_notification(title: &str, message: &str) {
    let script = format!(
        "[void] [Windows.UI.Notifications.ToastNotificationManager, Windows.UI.Notifications, ContentType = WindowsRuntime]; \
		 [void] [Windows.Data.Xml.Dom.XmlDocument, Windows.Data.Xml.Dom.XmlDocument, ContentType = WindowsRuntime]; \
		 $el = [Windows.UI.Notifications.ToastNotificationManager]::GetTemplateContent([Windows.UI.Notifications.ToastTemplateType]::ToastText02); \
		 $el.GetElementsByTagName('text').Item(0).InnerText = '{}'; \
		 $el.GetElementsByTagName('text').Item(1).InnerText = '{}'; \
		 $notifier = [Windows.UI.Notifications.ToastNotificationManager]::CreateToastNotifier('rtem'); \
		 $notifier.Show($el)",
        title.replace('\'', "''"),
        message.replace('\'', "''")
    );

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        let _ = std::process::Command::new("powershell")
            .args(["-NoProfile", "-Command", &script])
            .creation_flags(0x08000000) // CREATE_NO_WINDOW = 0x08000000
            .spawn();
    }
    #[cfg(not(windows))]
    {
        let _ = std::process::Command::new("powershell")
            .args(["-NoProfile", "-Command", &script])
            .spawn();
    }
}

/// Write a record directly to the native Windows Event Log under Application.
pub fn log_windows_event(source_name: &str, event_type: u16, event_id: u32, message: &str) {
    #[cfg(windows)]
    unsafe {
        let source_w: Vec<u16> = source_name
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let handle = RegisterEventSourceW(std::ptr::null(), source_w.as_ptr());
        if !handle.is_null() {
            let message_w: Vec<u16> = message.encode_utf16().chain(std::iter::once(0)).collect();
            let strings: [*const u16; 1] = [message_w.as_ptr()];

            ReportEventW(
                handle,
                event_type,
                0, // category
                event_id,
                std::ptr::null_mut(), // user sid
                1,                    // num strings
                0,                    // data size
                strings.as_ptr(),
                std::ptr::null_mut(), // raw data
            );
            DeregisterEventSource(handle);
        }
    }
}

#[derive(Debug, Clone)]
pub struct DiskDriveInfo {
    pub path: String,
    pub total_bytes: u64,
    pub free_bytes: u64,
}

/// Query storage capacities and free space of all active logical drives in real-time.
pub fn query_disk_drives() -> Vec<DiskDriveInfo> {
    let mut drives = Vec::new();
    #[cfg(windows)]
    unsafe {
        let mut buffer = [0u16; 512];
        let len = GetLogicalDriveStringsW(buffer.len() as u32, buffer.as_mut_ptr());
        if len > 0 && len < buffer.len() as u32 {
            let mut start = 0;
            for idx in 0..len as usize {
                if buffer[idx] == 0 {
                    if idx > start {
                        let drive_w = &buffer[start..idx];
                        let mut path_null = drive_w.to_vec();
                        path_null.push(0);

                        let mut free_caller = 0u64;
                        let mut total = 0u64;
                        let mut free_total = 0u64;
                        let ok = GetDiskFreeSpaceExW(
                            path_null.as_ptr(),
                            &mut free_caller,
                            &mut total,
                            &mut free_total,
                        );
                        if ok != 0 {
                            if let Ok(path) = String::from_utf16(drive_w) {
                                drives.push(DiskDriveInfo {
                                    path: path.trim_end_matches('\0').to_string(),
                                    total_bytes: total,
                                    free_bytes: free_caller,
                                });
                            }
                        }
                    }
                    start = idx + 1;
                }
            }
        }
    }
    drives
}

/// Query current state (RUNNING, STOPPED, etc.) of a specific Windows Service from SCM.
pub fn query_windows_service_status(service_name: &str) -> String {
    #[cfg(windows)]
    unsafe {
        let scm = OpenSCManagerW(std::ptr::null(), std::ptr::null(), 0x0001); // SC_MANAGER_CONNECT = 0x0001
        if !scm.is_null() {
            let service_name_w: Vec<u16> = service_name
                .encode_utf16()
                .chain(std::iter::once(0))
                .collect();
            let svc = OpenServiceW(scm, service_name_w.as_ptr(), 0x0004); // SERVICE_QUERY_STATUS = 0x0004
            if !svc.is_null() {
                let mut status = SERVICE_STATUS::default();
                let ok = QueryServiceStatus(svc, &mut status);
                CloseServiceHandle(svc);
                CloseServiceHandle(scm);
                if ok != 0 {
                    return match status.dwCurrentState {
                        1 => "STOPPED".to_string(),
                        2 => "START_PENDING".to_string(),
                        3 => "STOP_PENDING".to_string(),
                        4 => "RUNNING".to_string(),
                        5 => "CONTINUE_PENDING".to_string(),
                        6 => "PAUSE_PENDING".to_string(),
                        7 => "PAUSED".to_string(),
                        _ => "UNKNOWN".to_string(),
                    };
                }
            } else {
                CloseServiceHandle(scm);
            }
        }
    }
    "NOT_FOUND".to_string()
}

/// Dynamic platform-independent helper to query the primary host IP address.
pub fn query_local_ip() -> Option<String> {
    let socket = std::net::UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    socket.local_addr().ok().map(|addr| addr.ip().to_string())
}

/// Set the system clipboard text using native Win32 APIs.
pub fn copy_text_to_clipboard(text: &str) -> std::io::Result<()> {
    #[cfg(windows)]
    unsafe {
        use std::ptr;
        if OpenClipboard(ptr::null_mut()) == 0 {
            return Err(std::io::Error::last_os_error());
        }
        if EmptyClipboard() == 0 {
            let _ = CloseClipboard();
            return Err(std::io::Error::last_os_error());
        }

        let text_w: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
        let len = text_w.len() * 2;
        let h_mem = GlobalAlloc(0x0002, len); // GMEM_MOVEABLE = 0x0002
        if h_mem.is_null() {
            let _ = CloseClipboard();
            return Err(std::io::Error::last_os_error());
        }

        let ptr = GlobalLock(h_mem);
        if ptr.is_null() {
            let _ = GlobalFree(h_mem);
            let _ = CloseClipboard();
            return Err(std::io::Error::last_os_error());
        }

        std::ptr::copy_nonoverlapping(text_w.as_ptr(), ptr as *mut u16, text_w.len());
        GlobalUnlock(h_mem);

        if SetClipboardData(13, h_mem).is_null() {
            // CF_UNICODETEXT = 13
            let _ = GlobalFree(h_mem);
            let _ = CloseClipboard();
            return Err(std::io::Error::last_os_error());
        }

        CloseClipboard();
    }
    Ok(())
}

#[derive(Clone)]
pub struct WlanNetwork {
    pub ssid: String,
    pub signal_quality: u32,
    pub is_connected: bool,
    pub has_profile: bool,
    pub security_enabled: bool,
    pub auth_algorithm: String,
    pub cipher_algorithm: String,
    pub interface_guid: windows_sys::core::GUID,
}

impl std::fmt::Debug for WlanNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WlanNetwork")
            .field("ssid", &self.ssid)
            .field("signal_quality", &self.signal_quality)
            .field("is_connected", &self.is_connected)
            .field("has_profile", &self.has_profile)
            .field("security_enabled", &self.security_enabled)
            .field("auth_algorithm", &self.auth_algorithm)
            .field("cipher_algorithm", &self.cipher_algorithm)
            .field("interface_guid", &format!(
                "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                self.interface_guid.data1,
                self.interface_guid.data2,
                self.interface_guid.data3,
                self.interface_guid.data4[0],
                self.interface_guid.data4[1],
                self.interface_guid.data4[2],
                self.interface_guid.data4[3],
                self.interface_guid.data4[4],
                self.interface_guid.data4[5],
                self.interface_guid.data4[6],
                self.interface_guid.data4[7],
            ))
            .finish()
    }
}

impl PartialEq for WlanNetwork {
    fn eq(&self, other: &Self) -> bool {
        self.ssid == other.ssid
            && self.signal_quality == other.signal_quality
            && self.is_connected == other.is_connected
            && self.has_profile == other.has_profile
            && self.security_enabled == other.security_enabled
            && self.auth_algorithm == other.auth_algorithm
            && self.cipher_algorithm == other.cipher_algorithm
            && self.interface_guid.data1 == other.interface_guid.data1
            && self.interface_guid.data2 == other.interface_guid.data2
            && self.interface_guid.data3 == other.interface_guid.data3
            && self.interface_guid.data4 == other.interface_guid.data4
    }
}

impl Eq for WlanNetwork {}

pub fn query_wifi_networks(force_scan: bool) -> Result<Vec<WlanNetwork>, u32> {
    use windows_sys::Win32::NetworkManagement::WiFi::*;
    use windows_sys::Win32::Foundation::ERROR_SUCCESS;
    use windows_sys::core::GUID;

    let mut negotiated_version = 0;
    let mut handle = std::ptr::null_mut();
    
    let res = unsafe { WlanOpenHandle(2, std::ptr::null(), &mut negotiated_version, &mut handle) };
    if res != ERROR_SUCCESS {
        return Err(res);
    }

    let mut interface_list = std::ptr::null_mut();
    let res = unsafe { WlanEnumInterfaces(handle, std::ptr::null(), &mut interface_list) };
    if res != ERROR_SUCCESS {
        unsafe { WlanCloseHandle(handle, std::ptr::null_mut()) };
        return Err(res);
    }

    let interface_list_ref = unsafe { &*interface_list };

    if force_scan {
        for i in 0..interface_list_ref.dwNumberOfItems as usize {
            let interface_info = unsafe { *interface_list_ref.InterfaceInfo.as_ptr().add(i) };
            unsafe {
                WlanScan(
                    handle,
                    &interface_info.InterfaceGuid,
                    std::ptr::null(),
                    std::ptr::null(),
                    std::ptr::null(),
                );
            }
        }
        // Sleep to allow the hardware to complete the channel sweeps
        std::thread::sleep(std::time::Duration::from_millis(1200));
    }

    let mut networks = Vec::new();
    
    for i in 0..interface_list_ref.dwNumberOfItems as usize {
        let interface_info = unsafe { *interface_list_ref.InterfaceInfo.as_ptr().add(i) };
        
        let mut network_list = std::ptr::null_mut();
        let res = unsafe {
            WlanGetAvailableNetworkList(
                handle,
                &interface_info.InterfaceGuid,
                0,
                std::ptr::null(),
                &mut network_list,
            )
        };
        
        if res == ERROR_SUCCESS {
            let network_list_ref = unsafe { &*network_list };
            for j in 0..network_list_ref.dwNumberOfItems as usize {
                let net = unsafe { *network_list_ref.Network.as_ptr().add(j) };
                let ssid_len = net.dot11Ssid.uSSIDLength as usize;
                
                // Convert SSID bytes to string safely
                let ssid = String::from_utf8_lossy(&net.dot11Ssid.ucSSID[..ssid_len]).into_owned();
                
                // Skip empty SSID hidden networks for now
                if ssid.is_empty() {
                    continue;
                }

                // Check if this SSID is already in our list (avoid duplicates if detected on multiple interfaces)
                if networks.iter().any(|n: &WlanNetwork| n.ssid == ssid && n.is_connected) {
                    continue;
                }

                let is_connected = (net.dwFlags & WLAN_AVAILABLE_NETWORK_CONNECTED) != 0;
                let has_profile = (net.dwFlags & WLAN_AVAILABLE_NETWORK_HAS_PROFILE) != 0;
                
                let auth_algo = match net.dot11DefaultAuthAlgorithm {
                    DOT11_AUTH_ALGO_80211_OPEN => "Open",
                    DOT11_AUTH_ALGO_80211_SHARED_KEY => "Shared Key",
                    DOT11_AUTH_ALGO_WPA => "WPA-Enterprise",
                    DOT11_AUTH_ALGO_WPA_PSK => "WPA-Personal",
                    DOT11_AUTH_ALGO_RSNA => "WPA2-Enterprise",
                    DOT11_AUTH_ALGO_RSNA_PSK => "WPA2-Personal",
                    10 => "WPA3-Personal",
                    11 => "WPA3-Enterprise",
                    _ => "Unknown",
                }.to_string();

                let cipher_algo = match net.dot11DefaultCipherAlgorithm {
                    DOT11_CIPHER_ALGO_NONE => "None",
                    DOT11_CIPHER_ALGO_WEP => "WEP",
                    DOT11_CIPHER_ALGO_TKIP => "TKIP",
                    DOT11_CIPHER_ALGO_CCMP => "AES-CCMP",
                    100 => "GCMP-256",
                    _ => "Unknown",
                }.to_string();

                networks.push(WlanNetwork {
                    ssid,
                    signal_quality: net.wlanSignalQuality,
                    is_connected,
                    has_profile,
                    security_enabled: net.bSecurityEnabled != 0,
                    auth_algorithm: auth_algo,
                    cipher_algorithm: cipher_algo,
                    interface_guid: interface_info.InterfaceGuid,
                });
            }
            unsafe { WlanFreeMemory(network_list as *mut _) };
        }
    }

    unsafe {
        WlanFreeMemory(interface_list as *mut _);
        WlanCloseHandle(handle, std::ptr::null_mut());
    }

    // Sort networks so that connected is first, then by signal quality
    networks.sort_by(|a, b| {
        if a.is_connected && !b.is_connected {
            std::cmp::Ordering::Less
        } else if !a.is_connected && b.is_connected {
            std::cmp::Ordering::Greater
        } else {
            b.signal_quality.cmp(&a.signal_quality)
        }
    });

    Ok(networks)
}

fn escape_xml(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            _ => escaped.push(c),
        }
    }
    escaped
}

pub fn connect_to_wifi(ssid: &str, password: Option<&str>, net: &WlanNetwork) -> Result<(), String> {
    use windows_sys::Win32::NetworkManagement::WiFi::*;
    use windows_sys::Win32::Foundation::ERROR_SUCCESS;
    use windows_sys::core::GUID;

    let mut negotiated_version = 0;
    let mut handle = std::ptr::null_mut();
    
    let res = unsafe { WlanOpenHandle(2, std::ptr::null(), &mut negotiated_version, &mut handle) };
    if res != ERROR_SUCCESS {
        return Err(format!("WlanOpenHandle failed: {}", res));
    }

    let profile_name = ssid.to_string();

    if !net.has_profile {
        // We need to create an XML profile.
        let xml_profile = if net.security_enabled {
            let pwd = password.ok_or_else(|| "Password required for secured network".to_string())?;
            
            let (xml_auth, xml_cipher) = match net.auth_algorithm.as_str() {
                "WPA2-Personal" => ("WPA2PSK", "AES"),
                "WPA-Personal" => ("WPAPSK", "TKIP"),
                "WPA3-Personal" => ("WPA3SAE", "AES"),
                "Open" if net.security_enabled => ("WEP", "WEP"),
                _ => ("WPA2PSK", "AES"), // default fallback WPA2PSK is standard today
            };

            // Convert SSID to hex string
            let ssid_hex = ssid.as_bytes().iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join("");

            let escaped_ssid = escape_xml(ssid);
            let escaped_pwd = escape_xml(pwd);

            format!(
                r#"<?xml version="1.0"?>
<WLANProfile xmlns="http://www.microsoft.com/networking/WLAN/profile/v1">
    <name>{}</name>
    <SSIDConfig>
        <SSID>
            <hex>{}</hex>
            <name>{}</name>
        </SSID>
    </SSIDConfig>
    <connectionType>ESS</connectionType>
    <connectionMode>manual</connectionMode>
    <MSM>
        <security>
            <authEncryption>
                <authentication>{}</authentication>
                <encryption>{}</encryption>
                <useOneX>false</useOneX>
            </authEncryption>
            <sharedKey>
                <keyType>passPhrase</keyType>
                <protected>false</protected>
                <keyMaterial>{}</keyMaterial>
            </sharedKey>
        </security>
    </MSM>
</WLANProfile>"#,
                escaped_ssid, ssid_hex, escaped_ssid, xml_auth, xml_cipher, escaped_pwd
            )
        } else {
            // Open, unsecured network
            let ssid_hex = ssid.as_bytes().iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join("");
            let escaped_ssid = escape_xml(ssid);
            format!(
                r#"<?xml version="1.0"?>
<WLANProfile xmlns="http://www.microsoft.com/networking/WLAN/profile/v1">
    <name>{}</name>
    <SSIDConfig>
        <SSID>
            <hex>{}</hex>
            <name>{}</name>
        </SSID>
    </SSIDConfig>
    <connectionType>ESS</connectionType>
    <connectionMode>manual</connectionMode>
    <MSM>
        <security>
            <authEncryption>
                <authentication>open</authentication>
                <encryption>none</encryption>
                <useOneX>false</useOneX>
            </authEncryption>
        </security>
    </MSM>
</WLANProfile>"#,
                escaped_ssid, ssid_hex, escaped_ssid
            )
        };

        // Set the profile in Windows
        let xml_w: Vec<u16> = xml_profile.encode_utf16().chain(std::iter::once(0)).collect();
        let mut reason_code = 0;
        let res = unsafe {
            WlanSetProfile(
                handle,
                &net.interface_guid,
                0,
                xml_w.as_ptr(),
                std::ptr::null(),
                1,
                std::ptr::null(),
                &mut reason_code,
            )
        };

        if res != ERROR_SUCCESS {
            unsafe { WlanCloseHandle(handle, std::ptr::null_mut()) };
            return Err(format!("WlanSetProfile failed: code {}, reason {}", res, reason_code));
        }
    }

    // Now connect to the profile
    let profile_w: Vec<u16> = profile_name.encode_utf16().chain(std::iter::once(0)).collect();
    
    // We connect using connection mode: wlan_connection_mode_profile
    let mut conn_params: WLAN_CONNECTION_PARAMETERS = unsafe { std::mem::zeroed() };
    conn_params.wlanConnectionMode = wlan_connection_mode_profile;
    conn_params.strProfile = profile_w.as_ptr();
    conn_params.dot11BssType = dot11_BSS_type_infrastructure;

    let res = unsafe {
        WlanConnect(
            handle,
            &net.interface_guid,
            &conn_params,
            std::ptr::null(),
        )
    };

    unsafe { WlanCloseHandle(handle, std::ptr::null_mut()) };

    if res != ERROR_SUCCESS {
        return Err(format!("WlanConnect failed: {}", res));
    }

    Ok(())
}

pub fn disconnect_wifi(interface_guid: &windows_sys::core::GUID) -> Result<(), String> {
    use windows_sys::Win32::NetworkManagement::WiFi::*;
    use windows_sys::Win32::Foundation::ERROR_SUCCESS;

    let mut negotiated_version = 0;
    let mut handle = std::ptr::null_mut();
    
    let res = unsafe { WlanOpenHandle(2, std::ptr::null(), &mut negotiated_version, &mut handle) };
    if res != ERROR_SUCCESS {
        return Err(format!("WlanOpenHandle failed: {}", res));
    }

    let res = unsafe { WlanDisconnect(handle, interface_guid, std::ptr::null()) };
    unsafe { WlanCloseHandle(handle, std::ptr::null_mut()) };

    if res != ERROR_SUCCESS {
        return Err(format!("WlanDisconnect failed: {}", res));
    }

    Ok(())
}

pub fn delete_wifi_profile(ssid: &str, interface_guid: &windows_sys::core::GUID) -> Result<(), String> {
    use windows_sys::Win32::NetworkManagement::WiFi::*;
    use windows_sys::Win32::Foundation::ERROR_SUCCESS;

    let mut negotiated_version = 0;
    let mut handle = std::ptr::null_mut();
    
    let res = unsafe { WlanOpenHandle(2, std::ptr::null(), &mut negotiated_version, &mut handle) };
    if res != ERROR_SUCCESS {
        return Err(format!("WlanOpenHandle failed: {}", res));
    }

    let ssid_w: Vec<u16> = ssid.encode_utf16().chain(std::iter::once(0)).collect();
    let res = unsafe { WlanDeleteProfile(handle, interface_guid, ssid_w.as_ptr(), std::ptr::null()) };
    
    unsafe { WlanCloseHandle(handle, std::ptr::null_mut()) };

    if res != ERROR_SUCCESS {
        return Err(format!("WlanDeleteProfile failed: {}", res));
    }

    Ok(())
}

/// Helper to truncate a string to a max length and append "..." if exceeded.
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.chars().count() > max_len {
        let mut truncated: String = s.chars().take(max_len.saturating_sub(3)).collect();
        truncated.push_str("...");
        truncated
    } else {
        s.to_string()
    }
}

pub struct RadioState {
    pub software_on: bool,
    pub hardware_on: bool,
}

pub fn get_first_interface_guid() -> Result<windows_sys::core::GUID, u32> {
    use windows_sys::Win32::NetworkManagement::WiFi::*;
    use windows_sys::Win32::Foundation::ERROR_SUCCESS;

    let mut negotiated_version = 0;
    let mut handle = std::ptr::null_mut();
    
    let res = unsafe { WlanOpenHandle(2, std::ptr::null(), &mut negotiated_version, &mut handle) };
    if res != ERROR_SUCCESS {
        return Err(res);
    }

    let mut interface_list = std::ptr::null_mut();
    let res = unsafe { WlanEnumInterfaces(handle, std::ptr::null(), &mut interface_list) };
    if res != ERROR_SUCCESS {
        unsafe { WlanCloseHandle(handle, std::ptr::null_mut()) };
        return Err(res);
    }

    let interface_list_ref = unsafe { &*interface_list };
    if interface_list_ref.dwNumberOfItems == 0 {
        unsafe {
            WlanFreeMemory(interface_list as *mut _);
            WlanCloseHandle(handle, std::ptr::null_mut());
        }
        return Err(1168); // ERROR_NOT_FOUND = 1168
    }

    let interface_info = unsafe { *interface_list_ref.InterfaceInfo.as_ptr() };
    let guid = interface_info.InterfaceGuid;

    unsafe {
        WlanFreeMemory(interface_list as *mut _);
        WlanCloseHandle(handle, std::ptr::null_mut());
    }

    Ok(guid)
}

pub fn query_radio_state(interface_guid: &windows_sys::core::GUID) -> Result<RadioState, u32> {
    use windows_sys::Win32::NetworkManagement::WiFi::*;
    use windows_sys::Win32::Foundation::ERROR_SUCCESS;

    let mut negotiated_version = 0;
    let mut handle = std::ptr::null_mut();
    
    let res = unsafe { WlanOpenHandle(2, std::ptr::null(), &mut negotiated_version, &mut handle) };
    if res != ERROR_SUCCESS {
        return Err(res);
    }

    let mut data_size = 0;
    let mut data_ptr = std::ptr::null_mut();
    let mut opcode_value_type = wlan_opcode_value_type_query_only;

    let res = unsafe {
        WlanQueryInterface(
            handle,
            interface_guid,
            wlan_intf_opcode_radio_state,
            std::ptr::null(),
            &mut data_size,
            &mut data_ptr,
            &mut opcode_value_type,
        )
    };

    if res != ERROR_SUCCESS {
        unsafe { WlanCloseHandle(handle, std::ptr::null_mut()) };
        return Err(res);
    }

    let radio_state_ref = unsafe { &*(data_ptr as *const WLAN_RADIO_STATE) };
    
    let mut software_on = true;
    let mut hardware_on = true;

    if radio_state_ref.dwNumberOfPhys > 0 {
        let phy_state = radio_state_ref.PhyRadioState[0];
        software_on = phy_state.dot11SoftwareRadioState == dot11_radio_state_on;
        hardware_on = phy_state.dot11HardwareRadioState == dot11_radio_state_on;
    }

    unsafe {
        WlanFreeMemory(data_ptr);
        WlanCloseHandle(handle, std::ptr::null_mut());
    }

    Ok(RadioState { software_on, hardware_on })
}

pub fn set_radio_state(interface_guid: &windows_sys::core::GUID, software_on: bool) -> Result<(), u32> {
    use windows_sys::Win32::NetworkManagement::WiFi::*;
    use windows_sys::Win32::Foundation::ERROR_SUCCESS;

    let mut negotiated_version = 0;
    let mut handle = std::ptr::null_mut();
    
    let res = unsafe { WlanOpenHandle(2, std::ptr::null(), &mut negotiated_version, &mut handle) };
    if res != ERROR_SUCCESS {
        return Err(res);
    }

    let mut radio_state: WLAN_RADIO_STATE = unsafe { std::mem::zeroed() };
    radio_state.dwNumberOfPhys = 1;
    radio_state.PhyRadioState[0].dwPhyIndex = 0;
    radio_state.PhyRadioState[0].dot11SoftwareRadioState = if software_on {
        dot11_radio_state_on
    } else {
        dot11_radio_state_off
    };

    let res = unsafe {
        WlanSetInterface(
            handle,
            interface_guid,
            wlan_intf_opcode_radio_state,
            std::mem::size_of::<WLAN_RADIO_STATE>() as u32,
            &radio_state as *const _ as *const _,
            std::ptr::null(),
        )
    };

    unsafe { WlanCloseHandle(handle, std::ptr::null_mut()) };

    if res != ERROR_SUCCESS {
        return Err(res);
    }

    Ok(())
}

pub fn query_saved_password(ssid: &str) -> Option<String> {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        let output = std::process::Command::new("netsh")
            .args(["wlan", "show", "profile", &format!("name={}", ssid), "key=clear"])
            .creation_flags(0x08000000) // CREATE_NO_WINDOW
            .output()
            .ok()?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                let trimmed = line.trim();
                if trimmed.contains("Key Content") || trimmed.contains("Contenido de la clave") || trimmed.contains("Schlüsselinhalt") || trimmed.contains("Contenu de la") {
                    if let Some(pos) = trimmed.find(':') {
                        return Some(trimmed[pos + 1..].trim().to_string());
                    }
                }
            }
        }
    }
    None
}

pub fn query_saved_profiles() -> Result<Vec<(String, windows_sys::core::GUID)>, u32> {
    use windows_sys::Win32::NetworkManagement::WiFi::*;
    use windows_sys::Win32::Foundation::ERROR_SUCCESS;
    
    let mut negotiated_version = 0;
    let mut handle = std::ptr::null_mut();
    let res = unsafe { WlanOpenHandle(2, std::ptr::null(), &mut negotiated_version, &mut handle) };
    if res != ERROR_SUCCESS {
        return Err(res);
    }
    
    let mut interface_list = std::ptr::null_mut();
    let res = unsafe { WlanEnumInterfaces(handle, std::ptr::null(), &mut interface_list) };
    if res != ERROR_SUCCESS {
        unsafe { WlanCloseHandle(handle, std::ptr::null_mut()) };
        return Err(res);
    }
    
    let mut profiles = Vec::new();
    let interface_list_ref = unsafe { &*interface_list };
    
    for i in 0..interface_list_ref.dwNumberOfItems as usize {
        let interface_info = unsafe { *interface_list_ref.InterfaceInfo.as_ptr().add(i) };
        let mut profile_list = std::ptr::null_mut();
        let res = unsafe {
            WlanGetProfileList(
                handle,
                &interface_info.InterfaceGuid,
                std::ptr::null(),
                &mut profile_list,
            )
        };
        
        if res == ERROR_SUCCESS {
            let profile_list_ref = unsafe { &*profile_list };
            for j in 0..profile_list_ref.dwNumberOfItems as usize {
                let profile_info = unsafe { *profile_list_ref.ProfileInfo.as_ptr().add(j) };
                let len = profile_info.strProfileName.iter().position(|&c| c == 0).unwrap_or(profile_info.strProfileName.len());
                if let Ok(name) = String::from_utf16(&profile_info.strProfileName[..len]) {
                    if !name.is_empty() {
                        profiles.push((name, interface_info.InterfaceGuid));
                    }
                }
            }
            unsafe { WlanFreeMemory(profile_list as *mut _) };
        }
    }
    
    unsafe {
        WlanFreeMemory(interface_list as *mut _);
        WlanCloseHandle(handle, std::ptr::null_mut());
    }
    
    Ok(profiles)
}

pub fn connect_to_enterprise_wifi(
    ssid: &str,
    username: &str,
    password: &str,
    net: &WlanNetwork,
) -> Result<(), String> {
    use windows_sys::Win32::NetworkManagement::WiFi::*;
    use windows_sys::Win32::Foundation::ERROR_SUCCESS;

    let mut negotiated_version = 0;
    let mut handle = std::ptr::null_mut();
    
    let res = unsafe { WlanOpenHandle(2, std::ptr::null(), &mut negotiated_version, &mut handle) };
    if res != ERROR_SUCCESS {
        return Err(format!("WlanOpenHandle failed: {}", res));
    }

    let profile_name = ssid.to_string();

    let xml_auth = "WPA2";
    let xml_cipher = "AES";
    let ssid_hex = ssid.as_bytes().iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join("");
    let escaped_ssid = escape_xml(ssid);

    let profile_xml = format!(
        r#"<?xml version="1.0"?>
<WLANProfile xmlns="http://www.microsoft.com/networking/WLAN/profile/v1">
    <name>{}</name>
    <SSIDConfig>
        <SSID>
            <hex>{}</hex>
            <name>{}</name>
        </SSID>
    </SSIDConfig>
    <connectionType>ESS</connectionType>
    <connectionMode>manual</connectionMode>
    <MSM>
        <security>
            <authEncryption>
                <authentication>{}</authentication>
                <encryption>{}</encryption>
                <useOneX>true</useOneX>
            </authEncryption>
            <OneX xmlns="http://www.microsoft.com/networking/OneX/v1">
                <authMode>user</authMode>
                <EAPConfig>
                    <EapHostConfig xmlns="http://www.microsoft.com/networking/EapHostConfig">
                        <EapMethod>
                            <Type xmlns="http://www.microsoft.com/grouping/eapandcert/eapmethodconfig">25</Type>
                            <VendorId xmlns="http://www.microsoft.com/grouping/eapandcert/eapmethodconfig">0</VendorId>
                            <AuthorId xmlns="http://www.microsoft.com/grouping/eapandcert/eapmethodconfig">0</AuthorId>
                            <Type xmlns="http://www.microsoft.com/grouping/eapandcert/eapmethodconfig">0</Type>
                        </EapMethod>
                        <Config xmlns="http://www.microsoft.com/networking/EapHostConfig">
                            <EapType xmlns="http://www.microsoft.com/grouping/eapandcert/eapdeviceconfig">
                                <Type>25</Type>
                                <ConfigP xmlns="http://www.microsoft.com/grouping/eapandcert/eapdeviceconfig">
                                    <peapEapConnectionProperties xmlns="http://www.microsoft.com/provisioning/mspeapconnectionpropertiesv1">
                                        <PerformServerValidation>false</PerformServerValidation>
                                        <AcceptServerName>false</AcceptServerName>
                                        <FastReconnect>true</FastReconnect>
                                        <InnerEapOptional>false</InnerEapOptional>
                                        <Eap xmlns="http://www.microsoft.com/provisioning/baseeapconnectionpropertiesv1">
                                            <Type>26</Type>
                                            <Config xmlns="http://www.microsoft.com/provisioning/baseeapconnectionpropertiesv1">
                                                <EapType xmlns="http://www.microsoft.com/provisioning/mschapv2connectionpropertiesv1">
                                                    <UseWinLogonCredentials>false</UseWinLogonCredentials>
                                                </EapType>
                                            </Config>
                                        </Eap>
                                    </peapEapConnectionProperties>
                                </ConfigP>
                            </EapType>
                        </Config>
                    </EapHostConfig>
                </EAPConfig>
            </OneX>
        </security>
    </MSM>
</WLANProfile>"#,
        escaped_ssid, ssid_hex, escaped_ssid, xml_auth, xml_cipher
    );

    let xml_w: Vec<u16> = profile_xml.encode_utf16().chain(std::iter::once(0)).collect();
    let mut reason_code = 0;
    let res = unsafe {
        WlanSetProfile(
            handle,
            &net.interface_guid,
            0,
            xml_w.as_ptr(),
            std::ptr::null(),
            1,
            std::ptr::null(),
            &mut reason_code,
        )
    };

    if res != ERROR_SUCCESS {
        unsafe { WlanCloseHandle(handle, std::ptr::null_mut()) };
        return Err(format!("WlanSetProfile failed: code {}, reason {}", res, reason_code));
    }

    let escaped_user = escape_xml(username);
    let escaped_pwd = escape_xml(password);
    let user_xml = format!(
        r#"<EapHostUserCredentials xmlns="http://www.microsoft.com/networking/EapHostUserCredentials" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    <EapMethod>
        <Type xmlns="http://www.microsoft.com/grouping/eapandcert/eapmethodconfig">25</Type>
        <VendorId xmlns="http://www.microsoft.com/grouping/eapandcert/eapmethodconfig">0</VendorId>
        <AuthorId xmlns="http://www.microsoft.com/grouping/eapandcert/eapmethodconfig">0</AuthorId>
        <Type xmlns="http://www.microsoft.com/grouping/eapandcert/eapmethodconfig">0</Type>
    </EapMethod>
    <Credentials xmlns="http://www.microsoft.com/networking/EapHostUserCredentials">
        <EapType xmlns="http://www.microsoft.com/grouping/eapandcert/eapdeviceconfig">
            <Type>25</Type>
            <ConfigP xmlns="http://www.microsoft.com/grouping/eapandcert/eapdeviceconfig">
                <peapUserProperties xmlns="http://www.microsoft.com/provisioning/mspeapuserpropertiesv1">
                    <RoutingIdentity xsi:nil="true"/>
                    <Eap xmlns="http://www.microsoft.com/provisioning/baseeapuserpropertiesv1">
                        <Type>26</Type>
                        <Config xmlns="http://www.microsoft.com/provisioning/baseeapuserpropertiesv1">
                            <EapType xmlns="http://www.microsoft.com/provisioning/mschapv2userpropertiesv1">
                                <Username>{}</Username>
                                <Password>{}</Password>
                                <LogonDomain></LogonDomain>
                            </EapType>
                        </Config>
                    </Eap>
                </peapUserProperties>
            </ConfigP>
        </EapType>
    </Credentials>
</EapHostUserCredentials>"#,
        escaped_user, escaped_pwd
    );

    let user_w: Vec<u16> = user_xml.encode_utf16().chain(std::iter::once(0)).collect();
    let profile_w: Vec<u16> = profile_name.encode_utf16().chain(std::iter::once(0)).collect();

    #[link(name = "wlanapi")]
    unsafe extern "system" {
        fn WlanSetProfileEapXmlUserData(
            hClientHandle: *mut std::ffi::c_void,
            pInterfaceGuid: *const windows_sys::core::GUID,
            strProfileName: *const u16,
            dwFlags: u32,
            strEapXmlUserData: *const u16,
            pReserved: *mut std::ffi::c_void,
        ) -> u32;
    }

    let res = unsafe {
        WlanSetProfileEapXmlUserData(
            handle,
            &net.interface_guid,
            profile_w.as_ptr(),
            0,
            user_w.as_ptr(),
            std::ptr::null_mut(),
        )
    };

    if res != ERROR_SUCCESS {
        unsafe { WlanCloseHandle(handle, std::ptr::null_mut()) };
        return Err(format!("WlanSetProfileEapXmlUserData failed: code {}", res));
    }

    let mut conn_params: WLAN_CONNECTION_PARAMETERS = unsafe { std::mem::zeroed() };
    conn_params.wlanConnectionMode = wlan_connection_mode_profile;
    conn_params.strProfile = profile_w.as_ptr();
    conn_params.dot11BssType = dot11_BSS_type_infrastructure;

    let res = unsafe {
        WlanConnect(
            handle,
            &net.interface_guid,
            &conn_params,
            std::ptr::null(),
        )
    };

    unsafe { WlanCloseHandle(handle, std::ptr::null_mut()) };

    if res != ERROR_SUCCESS {
        return Err(format!("WlanConnect failed: {}", res));
    }

    Ok(())
}

pub fn connect_to_hidden_wifi(
    ssid: &str,
    password: Option<&str>,
    security_enabled: bool,
    auth_algo: &str,
    _cipher_algo: &str,
    interface_guid: &windows_sys::core::GUID,
) -> Result<(), String> {
    use windows_sys::Win32::NetworkManagement::WiFi::*;
    use windows_sys::Win32::Foundation::ERROR_SUCCESS;

    let mut negotiated_version = 0;
    let mut handle = std::ptr::null_mut();
    
    let res = unsafe { WlanOpenHandle(2, std::ptr::null(), &mut negotiated_version, &mut handle) };
    if res != ERROR_SUCCESS {
        return Err(format!("WlanOpenHandle failed: {}", res));
    }

    let profile_name = ssid.to_string();

    let xml_profile = if security_enabled {
        let pwd = password.ok_or_else(|| "Password required for secured network".to_string())?;
        
        let (xml_auth, xml_cipher) = match auth_algo {
            "WPA2-Personal" | "WPA2PSK" => ("WPA2PSK", "AES"),
            "WPA-Personal" | "WPAPSK" => ("WPAPSK", "TKIP"),
            "WPA3-Personal" | "WPA3SAE" => ("WPA3SAE", "AES"),
            "Open" if security_enabled => ("WEP", "WEP"),
            _ => ("WPA2PSK", "AES"),
        };

        let ssid_hex = ssid.as_bytes().iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join("");
        let escaped_ssid = escape_xml(ssid);
        let escaped_pwd = escape_xml(pwd);

        format!(
            r#"<?xml version="1.0"?>
<WLANProfile xmlns="http://www.microsoft.com/networking/WLAN/profile/v1">
    <name>{}</name>
    <SSIDConfig>
        <SSID>
            <hex>{}</hex>
            <name>{}</name>
        </SSID>
        <nonBroadcast>true</nonBroadcast>
    </SSIDConfig>
    <connectionType>ESS</connectionType>
    <connectionMode>manual</connectionMode>
    <MSM>
        <security>
            <authEncryption>
                <authentication>{}</authentication>
                <encryption>{}</encryption>
                <useOneX>false</useOneX>
            </authEncryption>
            <sharedKey>
                <keyType>passPhrase</keyType>
                <protected>false</protected>
                <keyMaterial>{}</keyMaterial>
            </sharedKey>
        </security>
    </MSM>
</WLANProfile>"#,
            escaped_ssid, ssid_hex, escaped_ssid, xml_auth, xml_cipher, escaped_pwd
        )
    } else {
        let ssid_hex = ssid.as_bytes().iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join("");
        let escaped_ssid = escape_xml(ssid);
        format!(
            r#"<?xml version="1.0"?>
<WLANProfile xmlns="http://www.microsoft.com/networking/WLAN/profile/v1">
    <name>{}</name>
    <SSIDConfig>
        <SSID>
            <hex>{}</hex>
            <name>{}</name>
        </SSID>
        <nonBroadcast>true</nonBroadcast>
    </SSIDConfig>
    <connectionType>ESS</connectionType>
    <connectionMode>manual</connectionMode>
    <MSM>
        <security>
            <authEncryption>
                <authentication>open</authentication>
                <encryption>none</encryption>
                <useOneX>false</useOneX>
            </authEncryption>
        </security>
    </MSM>
</WLANProfile>"#,
            escaped_ssid, ssid_hex, escaped_ssid
        )
    };

    let xml_w: Vec<u16> = xml_profile.encode_utf16().chain(std::iter::once(0)).collect();
    let mut reason_code = 0;
    let res = unsafe {
        WlanSetProfile(
            handle,
            interface_guid,
            0,
            xml_w.as_ptr(),
            std::ptr::null(),
            1,
            std::ptr::null(),
            &mut reason_code,
        )
    };

    if res != ERROR_SUCCESS {
        unsafe { WlanCloseHandle(handle, std::ptr::null_mut()) };
        return Err(format!("WlanSetProfile failed: code {}, reason {}", res, reason_code));
    }

    let profile_w: Vec<u16> = profile_name.encode_utf16().chain(std::iter::once(0)).collect();
    let mut conn_params: WLAN_CONNECTION_PARAMETERS = unsafe { std::mem::zeroed() };
    conn_params.wlanConnectionMode = wlan_connection_mode_profile;
    conn_params.strProfile = profile_w.as_ptr();
    conn_params.dot11BssType = dot11_BSS_type_infrastructure;

    let res = unsafe {
        WlanConnect(
            handle,
            interface_guid,
            &conn_params,
            std::ptr::null(),
        )
    };

    unsafe { WlanCloseHandle(handle, std::ptr::null_mut()) };

    if res != ERROR_SUCCESS {
        return Err(format!("WlanConnect failed: {}", res));
    }

    Ok(())
}
