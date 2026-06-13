//! Keypress helpers for WiFi action execution.

use crate::app::AppState;
use crate::win32;

pub fn handle_disconnect(app: &mut AppState, net: &win32::WlanNetwork) {
    app.set_status(format!("Disconnecting from {}...", net.ssid), false);
    match win32::disconnect_wifi(&net.interface_guid) {
        Ok(_) => {
            app.set_status(format!("Disconnected from {}", net.ssid), false);
            win32::show_toast_notification("scout - Disconnected", &format!("Disconnected from {}", net.ssid));
            app.scan_wifi(false);
        }
        Err(e) => {
            app.set_status(format!("Failed to disconnect: {}", e), true);
        }
    }
}

pub fn handle_connect_open(app: &mut AppState, net: &win32::WlanNetwork) {
    app.set_status(format!("Connecting to {}...", net.ssid), false);
    match win32::connect_to_wifi(&net.ssid, None, net) {
        Ok(_) => {
            app.set_status(format!("Successfully connected to {}", net.ssid), false);
            win32::show_toast_notification("scout - Connected", &format!("Connected to {}", net.ssid));
            win32::log_windows_event("scout", 4, 1001, &format!("Successfully connected to {}", net.ssid));
            app.scan_wifi(false);
        }
        Err(e) => {
            app.set_status(format!("Failed to connect: {}", e), true);
            win32::show_toast_notification("scout - Connection Failed", &format!("Failed to connect to {}: {}", net.ssid, e));
        }
    }
}

pub fn handle_connect_secure(app: &mut AppState, net: &win32::WlanNetwork) {
    let is_enterprise = net.auth_algorithm.contains("Enterprise") || net.auth_algorithm.contains("RSNA");
    app.show_password_overlay = true;
    app.password_box.active = true;
    if is_enterprise {
        app.eap_prompt_username = true;
        app.password_visible = true;
        app.password_box.clear();
    } else {
        app.eap_prompt_username = false;
        app.password_visible = false;
        app.password_box.clear();
    }
}

pub fn handle_delete_profile(app: &mut AppState, net: &win32::WlanNetwork) {
    if net.is_connected {
        app.set_status(format!("Disconnecting from {}...", net.ssid), false);
        let _ = win32::disconnect_wifi(&net.interface_guid);
        app.scan_wifi(false);
    } else if net.has_profile {
        app.set_status(format!("Deleting profile for {}...", net.ssid), false);
        match win32::delete_wifi_profile(&net.ssid, &net.interface_guid) {
            Ok(_) => {
                app.set_status(format!("Deleted profile for {}", net.ssid), false);
                app.scan_wifi(false);
            }
            Err(e) => {
                app.set_status(format!("Failed to delete profile: {}", e), true);
            }
        }
    }
}

pub fn handle_toggle_radio(app: &mut AppState) {
    if let Ok(guid) = win32::get_first_interface_guid() {
        if let Ok(state) = win32::query_radio_state(&guid) {
            let target_state = !state.software_on;
            app.set_status(format!("Toggling Wi-Fi Radio to {}...", if target_state { "On" } else { "Off" }), false);
            match win32::set_radio_state(&guid, target_state) {
                Ok(_) => {
                    app.set_status(format!("Wi-Fi Radio successfully turned {}.", if target_state { "On" } else { "Off" }), false);
                    win32::show_toast_notification(
                        "scout - Radio Toggled",
                        &format!("Wi-Fi Radio turned {}", if target_state { "On" } else { "Off" })
                    );
                    app.scan_wifi(true);
                }
                Err(e) => {
                    app.set_status(format!("Failed to toggle Wi-Fi radio: Code {}", e), true);
                }
            }
        } else {
            app.set_status("Failed to query Wi-Fi radio state.".to_string(), true);
        }
    } else {
        app.set_status("No wireless interface found to toggle.".to_string(), true);
    }
}

pub fn handle_share_qr(app: &mut AppState, net: &win32::WlanNetwork) {
    app.share_ssid = net.ssid.clone();
    app.share_auth = net.auth_algorithm.clone();
    app.share_qr_lines.clear();
    if net.security_enabled {
        if let Some(password) = win32::query_saved_password(&net.ssid) {
            app.share_qr_lines = crate::ui::layout::generate_qr_code_lines(&net.ssid, &password, &net.auth_algorithm);
            app.show_share_overlay = true;
            app.share_prompt_password = false;
            app.share_box.active = false;
            app.set_status(format!("Generated QR code for network {} (loaded saved password).", net.ssid), false);
        } else {
            app.show_share_overlay = true;
            app.share_prompt_password = true;
            app.share_box.clear();
            app.share_box.active = true;
            app.set_status(format!("Enter password to generate QR code for network {}.", net.ssid), false);
        }
    } else {
        app.share_qr_lines = crate::ui::layout::generate_qr_code_lines(&net.ssid, "", &net.auth_algorithm);
        app.show_share_overlay = true;
        app.share_prompt_password = false;
        app.share_box.active = false;
        app.set_status(format!("Generated QR code for open network {}.", net.ssid), false);
    }
}

pub fn handle_saved_profiles(app: &mut AppState) {
    app.set_status("Querying saved offline network profiles...".to_string(), false);
    match win32::query_saved_profiles() {
        Ok(list) => {
            app.profiles_list = list;
            app.profiles_selected_idx = 0;
            app.show_profiles_overlay = true;
            app.set_status("Known profiles loaded. Press 'd' or 'Delete' to remove profile.".to_string(), false);
        }
        Err(e) => {
            app.set_status(format!("Failed to retrieve offline profiles: Code {}", e), true);
        }
    }
}
