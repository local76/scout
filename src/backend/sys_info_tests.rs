use super::*;

#[test]
fn test_power_status() {
    let ps = PowerStatus::default();
    assert!(ps.ac_online);
    assert_eq!(ps.battery_percent, 100);
    assert!(!ps.is_battery_percent_unknown());

    let ps_unknown = PowerStatus {
        ac_online: false,
        battery_percent: PowerStatus::BATTERY_PERCENT_UNKNOWN,
    };
    assert!(ps_unknown.is_battery_percent_unknown());
}

#[test]
fn test_system_bios_info() {
    let sbi = SystemBiosInfo::default();
    assert!(sbi.manufacturer.is_empty());
    assert!(sbi.product.is_empty());
    assert!(sbi.model.is_empty());
}

#[test]
fn test_glyph_map_load() {
    let map = GlyphMap::load();
    assert!(!map.status_ok.is_empty());
    assert!(!map.status_err.is_empty());
    assert!(!map.info.is_empty());
    assert!(!map.warning.is_empty());
    assert!(!map.cpu.is_empty());
    assert!(!map.gpu.is_empty());
    assert!(!map.memory.is_empty());
    assert!(!map.disk.is_empty());
    assert!(!map.package.is_empty());
    assert!(!map.battery.is_empty());
    assert!(!map.shell.is_empty());
    assert!(!map.terminal.is_empty());
    assert!(!map.network.is_empty());
    assert!(!map.clipboard.is_empty());
    assert!(!map.play.is_empty());
    assert!(!map.play_empty.is_empty());
}

#[test]
fn test_get_dwm_accent_color() {
    let color = get_dwm_accent_color();
    // Verify it is a valid color (can construct and matches RGB/fallback)
    match color {
        ratatui::style::Color::Rgb(..) => {}
        _ => panic!("Expected RGB color"),
    }
}

#[test]
fn test_query_local_ip() {
    // Should run without crashing; might return None or Some(IP) depending on network
    if let Some(ip) = query_local_ip() {
        assert!(!ip.is_empty());
    }
}
