use std::process::Command;
use super::{WlanNetwork, stub::get_wifi_interface};

pub fn connect_to_wifi(ssid: &str, password: Option<&str>, _net: &WlanNetwork) -> Result<(), String> {
    let mut args = vec!["device", "wifi", "connect", ssid];
    if let Some(pwd) = password {
        if !pwd.is_empty() {
            args.push("password");
            args.push(pwd);
        }
    }
    
    let output = Command::new("nmcli").args(&args).output();
    match output {
        Ok(out) => {
            if out.status.success() {
                Ok(())
            } else {
                Err(String::from_utf8_lossy(&out.stderr).into_owned())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub fn connect_to_enterprise_wifi(
    ssid: &str,
    username: &str,
    password: &str,
    _net: &WlanNetwork,
) -> Result<(), String> {
    let iface = get_wifi_interface();
    let con_name = ssid;
    let _ = Command::new("nmcli").args(["connection", "delete", con_name]).status();

    let output = Command::new("nmcli")
        .args([
            "connection", "add", "type", "wifi",
            "con-name", con_name,
            "ifname", &iface,
            "ssid", ssid,
            "--",
            "wifi-sec.key-mgmt", "wpa-eap",
            "802-1x.identity", username,
            "802-1x.password", password,
            "802-1x.eap", "peap",
            "802-1x.phase2-auth", "mschapv2"
        ])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let up_output = Command::new("nmcli")
                .args(["connection", "up", "id", con_name])
                .output();
            match up_output {
                Ok(up_out) if up_out.status.success() => Ok(()),
                Ok(up_out) => Err(String::from_utf8_lossy(&up_out.stderr).into_owned()),
                Err(e) => Err(e.to_string()),
            }
        }
        Ok(out) => Err(String::from_utf8_lossy(&out.stderr).into_owned()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn connect_to_hidden_wifi(
    ssid: &str,
    password: Option<&str>,
    security_enabled: bool,
    _auth_algo: &str,
    _cipher_algo: &str,
    _interface_guid: &super::windows_sys::core::GUID,
) -> Result<(), String> {
    let mut args = vec!["device", "wifi", "connect", ssid, "hidden", "yes"];
    if security_enabled {
        if let Some(pwd) = password {
            if !pwd.is_empty() {
                args.push("password");
                args.push(pwd);
            }
        }
    }
    
    let output = Command::new("nmcli").args(&args).output();
    match output {
        Ok(out) => {
            if out.status.success() {
                Ok(())
            } else {
                Err(String::from_utf8_lossy(&out.stderr).into_owned())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
