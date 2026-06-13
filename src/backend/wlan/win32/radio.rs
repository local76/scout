//! Windows WiFi radio hardware and software state management.
//!
//! **Taxonomy Classification**: Platform (WLAN / Win32 Radio).

use windows_sys::Win32::NetworkManagement::WiFi::*;
use windows_sys::Win32::Foundation::ERROR_SUCCESS;
use super::super::RadioState;

pub fn query_radio_state(interface_guid: &windows_sys::core::GUID) -> Result<RadioState, u32> {
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
