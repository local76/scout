//! Windows WiFi profile and password query implementation.
//!
//! **Taxonomy Classification**: Platform (WLAN / Win32 Profiles).

use windows_sys::Win32::NetworkManagement::WiFi::*;
use windows_sys::Win32::Foundation::ERROR_SUCCESS;

pub fn query_saved_password(ssid: &str) -> Option<String> {
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
            if (trimmed.contains("Key Content") || trimmed.contains("Contenido de la clave") || trimmed.contains("Schlüsselinhalt") || trimmed.contains("Contenu de la"))
                && let Some(pos) = trimmed.find(':') {
                    return Some(trimmed[pos + 1..].trim().to_string());
                }
        }
    }
    None
}

pub fn query_saved_profiles() -> Result<Vec<(String, windows_sys::core::GUID)>, u32> {
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
                if let Ok(name) = String::from_utf16(&profile_info.strProfileName[..len])
                    && !name.is_empty() {
                        profiles.push((name, interface_info.InterfaceGuid));
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
