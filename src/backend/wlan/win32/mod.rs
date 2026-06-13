//! Windows-specific WiFi query implementations.
//!
//! **Taxonomy Classification**: Platform (WLAN / Win32 Query).

pub mod connect;
pub mod connect_hidden;
pub mod enterprise;
pub mod radio;
pub mod profiles;

pub use connect::*;
pub use connect_hidden::*;
pub use enterprise::*;
pub use radio::*;
pub use profiles::*;

use super::{WlanNetwork, RadioState};

pub fn query_wifi_networks(force_scan: bool) -> Result<Vec<WlanNetwork>, u32> {
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
