//! Keyboard input event handler.
//!
//! **Taxonomy Classification**: Controller (Keyboard Controller).

use crossterm::event::KeyCode;
use crate::app::AppState;
use crate::ui::{ThemeColors, parse_markdown_to_lines};
use crate::win32;

pub fn handle_keypress(app: &mut AppState, code: KeyCode, theme: &ThemeColors) {
    if crate::app::keys_overlays::handle_overlay_keypress(app, code, theme) {
        return;
    }

    // Global override for F1-F7 documentation keys
    if let Some(name) = crate::chrome::embedded_docs::open_embedded_markdown(code) {
        app.show_help = false;
        app.show_markdown = Some(name.to_string());
        app.markdown_lines = parse_markdown_to_lines(doc_content(name), theme);
        app.markdown_scroll = 0;
        return;
    }

    if app.show_help {
        if matches!(code, KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Char('h')) {
            app.show_help = false;
        }
        return;
    }

    if app.show_markdown.is_some() {
        match code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                app.show_markdown = None;
                app.markdown_lines.clear();
            }
            KeyCode::Up | KeyCode::Char('k') => {
                app.markdown_scroll = app.markdown_scroll.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let (_, term_h) = crossterm::terminal::size().unwrap_or((110, 38));
                let inner_h = ((term_h * 80) / 100).saturating_sub(2) as usize;
                let max_scroll = app.markdown_lines.len().saturating_sub(inner_h);
                if app.markdown_scroll < max_scroll {
                    app.markdown_scroll += 1;
                }
            }
            _ => {}
        }
        return;
    }

    let filtered_nets: Vec<win32::WlanNetwork> = if app.search_active {
        app.networks
            .iter()
            .filter(|n| n.ssid.to_lowercase().contains(&app.search_box.text.to_lowercase()))
            .cloned()
            .collect()
    } else {
        app.networks.clone()
    };

    if app.search_active {
        match code {
            KeyCode::Esc => {
                app.search_active = false;
                app.search_box.active = false;
                app.search_box.clear();
                app.selected_network_idx = 0;
                app.set_status("Filter cleared.".to_string(), false);
                return;
            }
            KeyCode::Enter | KeyCode::Up | KeyCode::Down | KeyCode::Tab => {
                // Fall through
            }
            other => {
                app.search_box.handle_key(other);
                app.selected_network_idx = 0;
                return;
            }
        }
    }

    match code {
        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
            app.should_quit = true;
        }
        KeyCode::Tab => {
            app.focus = match app.focus {
                crate::app::FocusedSection::NetworkList => crate::app::FocusedSection::InfoPanel,
                crate::app::FocusedSection::InfoPanel => crate::app::FocusedSection::NetworkList,
            };
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.selected_network_idx > 0 {
                app.selected_network_idx -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.selected_network_idx + 1 < filtered_nets.len() {
                app.selected_network_idx += 1;
            }
        }
        KeyCode::Char('r') | KeyCode::Char('R') | KeyCode::Char(' ') => {
            app.set_status("Scanning Wi-Fi networks... Please wait.".to_string(), false);
            app.scan_wifi(true);
        }
        KeyCode::Enter => {
            if let Some(net) = filtered_nets.get(app.selected_network_idx).cloned() {
                if app.search_active {
                    let selected_ssid = net.ssid.clone();
                    app.search_active = false;
                    app.search_box.active = false;
                    app.search_box.clear();
                    if let Some(idx) = app.networks.iter().position(|n| n.ssid == selected_ssid) {
                        app.selected_network_idx = idx;
                    }
                }
                if net.is_connected {
                    crate::app::keys_wifi::handle_disconnect(app, &net);
                } else if !net.security_enabled {
                    crate::app::keys_wifi::handle_connect_open(app, &net);
                } else {
                    crate::app::keys_wifi::handle_connect_secure(app, &net);
                }
            }
        }
        KeyCode::Char('d') | KeyCode::Char('D') | KeyCode::Delete => {
            if let Some(net) = filtered_nets.get(app.selected_network_idx).cloned() {
                if app.search_active {
                    let selected_ssid = net.ssid.clone();
                    app.search_active = false;
                    app.search_box.active = false;
                    app.search_box.clear();
                    if let Some(idx) = app.networks.iter().position(|n| n.ssid == selected_ssid) {
                        app.selected_network_idx = idx;
                    }
                }
                crate::app::keys_wifi::handle_delete_profile(app, &net);
            }
        }
        KeyCode::Char('/') => {
            app.search_active = true;
            app.search_box.active = true;
            app.search_box.clear();
            app.selected_network_idx = 0;
            app.set_status("Search mode active. Type SSID to filter.".to_string(), false);
        }
        KeyCode::Char('t') | KeyCode::Char('T') => {
            crate::app::keys_wifi::handle_toggle_radio(app);
        }
        KeyCode::Char('s') | KeyCode::Char('S') => {
            if let Some(net) = filtered_nets.get(app.selected_network_idx).cloned() {
                crate::app::keys_wifi::handle_share_qr(app, &net);
            }
        }
        KeyCode::Char('p') | KeyCode::Char('P') => {
            crate::app::keys_wifi::handle_saved_profiles(app);
        }
        KeyCode::Char('w') | KeyCode::Char('W') => {
            app.show_hidden_overlay = true;
            app.hidden_ssid.clear();
            app.hidden_box.clear();
            app.hidden_box.active = true;
            app.hidden_prompt_step = 1;
            app.set_status("Enter hidden network SSID.".to_string(), false);
        }
        KeyCode::Char('h') => {
            app.show_help = true;
        }
        _ => {}
    }
}

/// Resolve a doc filename (e.g. "README.md") to its embedded markdown content.
fn doc_content(name: &str) -> &'static str {
    match name {
        "README.md" => crate::README_CONTENT,
        "SUPPORT.md" => crate::SUPPORT_CONTENT,
        "LICENSE.md" => crate::LICENSE_CONTENT,
        "COPYRIGHT.md" => crate::COPYRIGHT_CONTENT,
        "PRIVACY.md" => crate::PRIVACY_CONTENT,
        "SECURITY.md" => crate::SECURITY_CONTENT,
        "CONTRIBUTING.md" => crate::CONTRIBUTING_CONTENT,
        _ => "",
    }
}
