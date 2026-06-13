//! Render components for all UI widgets and overlay dialogues.
//!
//! **Taxonomy Classification**: UI Rendering (UI Panels).

use ratatui::{
    Frame,
    layout::{Rect, Margin},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use crate::app::{AppState, FocusedSection};
use crate::ui::ThemeColors;
use crate::win32;

pub fn draw_network_list(f: &mut Frame, app: &mut AppState, theme: &ThemeColors, area: Rect) {
    let left_border = if app.focus == FocusedSection::NetworkList { theme.border_active } else { theme.border };
    let left_title = if app.search_active {
        format!(" Available Wi-Fi Networks [Filter: {}_] ", app.search_box.text)
    } else {
        " Available Wi-Fi Networks ".to_string()
    };
    let left_block = Block::default()
        .borders(Borders::ALL)
        .title(left_title)
        .title_style(Style::default().fg(left_border).add_modifier(Modifier::BOLD))
        .border_style(Style::default().fg(left_border));

    let filtered_nets: Vec<win32::WlanNetwork> = if app.search_active {
        app.networks
            .iter()
            .filter(|n| n.ssid.to_lowercase().contains(&app.search_box.text.to_lowercase()))
            .cloned()
            .collect()
    } else {
        app.networks.clone()
    };

    let mut list_lines = Vec::new();
    if filtered_nets.is_empty() {
        list_lines.push(Line::from(""));
        if app.is_scanning {
            list_lines.push(Line::from(Span::styled("  Scanning for Wi-Fi networks... Please wait.", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD))));
        } else if app.search_active {
            list_lines.push(Line::from(Span::styled("  No matching Wi-Fi networks found. Press Esc to clear filter.", Style::default().fg(theme.text_dim))));
        } else {
            list_lines.push(Line::from(Span::styled("  No wireless stations discovered. Press Space to refresh.", Style::default().fg(theme.text_dim))));
        }
    } else {
        for (idx, net) in filtered_nets.iter().enumerate() {
            let is_selected = idx == app.selected_network_idx;
            
            let bullet = if net.is_connected {
                " * "
            } else if is_selected {
                app.glyphs.play
            } else {
                app.glyphs.play_empty
            };

            let bullet_style = if net.is_connected {
                Style::default().fg(Color::Rgb(80, 250, 80)).add_modifier(Modifier::BOLD)
            } else if is_selected {
                Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.text_dim)
            };

            let name_style = if is_selected {
                Style::default().fg(theme.text_main).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.text_dim)
            };

            // Format signal strength bar
            let bars_filled = (net.signal_quality as f32 / 20.0).round() as usize;
            let bar_char = if app.glyphs.status_ok == "✔️" { "█" } else { "#" };
            let empty_char = if app.glyphs.status_ok == "✔️" { "░" } else { "-" };
            let bar_str = format!(
                "[{}{}]",
                bar_char.repeat(bars_filled),
                empty_char.repeat(5 - bars_filled)
            );
            let bar_color = if net.signal_quality > 70 {
                Color::Rgb(80, 250, 80)
            } else if net.signal_quality > 40 {
                Color::Rgb(250, 250, 80)
            } else {
                Color::Rgb(250, 80, 80)
            };

            let lock_icon = if net.security_enabled { " [Secured]" } else { " [Open]" };
            let lock_color = if net.security_enabled { Color::Rgb(250, 180, 80) } else { Color::Rgb(80, 250, 80) };

            list_lines.push(Line::from(vec![
                Span::styled(bullet, bullet_style),
                Span::styled(format!("{:<25}", win32::truncate(&net.ssid, 25)), name_style),
                Span::styled(format!(" {:>3}% ", net.signal_quality), Style::default().fg(theme.text_main)),
                Span::styled(bar_str, Style::default().fg(bar_color)),
                Span::styled(lock_icon, Style::default().fg(lock_color)),
            ]));
        }
    }

    f.render_widget(Paragraph::new(list_lines).block(left_block), area);
}

pub fn draw_info_panel(f: &mut Frame, app: &mut AppState, theme: &ThemeColors, area: Rect) {
    let right_border = if app.focus == FocusedSection::InfoPanel { theme.border_active } else { theme.border };
    let right_block = Block::default()
        .borders(Borders::ALL)
        .title(" Connection Details ")
        .title_style(Style::default().fg(right_border).add_modifier(Modifier::BOLD))
        .border_style(Style::default().fg(right_border));

    let filtered_nets: Vec<win32::WlanNetwork> = if app.search_active {
        app.networks
            .iter()
            .filter(|n| n.ssid.to_lowercase().contains(&app.search_box.text.to_lowercase()))
            .cloned()
            .collect()
    } else {
        app.networks.clone()
    };

    let mut right_lines = Vec::new();
    if let Some(net) = filtered_nets.get(app.selected_network_idx) {
        right_lines.push(Line::from(""));
        right_lines.push(Line::from(vec![
            Span::styled("  SSID:        ", Style::default().fg(theme.text_dim)),
            Span::styled(&net.ssid, Style::default().fg(theme.text_main).add_modifier(Modifier::BOLD)),
        ]));
        
        let conn_status = if net.is_connected { "Connected" } else { "Disconnected" };
        let conn_color = if net.is_connected { Color::Rgb(80, 250, 80) } else { theme.text_dim };
        right_lines.push(Line::from(vec![
            Span::styled("  Status:      ", Style::default().fg(theme.text_dim)),
            Span::styled(conn_status, Style::default().fg(conn_color).add_modifier(Modifier::BOLD)),
        ]));

        right_lines.push(Line::from(vec![
            Span::styled("  Signal:      ", Style::default().fg(theme.text_dim)),
            Span::styled(format!("{}% ", net.signal_quality), Style::default().fg(theme.text_main)),
        ]));

        right_lines.push(Line::from(vec![
            Span::styled("  Security:    ", Style::default().fg(theme.text_dim)),
            Span::styled(&net.auth_algorithm, Style::default().fg(theme.text_main)),
        ]));

        right_lines.push(Line::from(vec![
            Span::styled("  Encryption:  ", Style::default().fg(theme.text_dim)),
            Span::styled(&net.cipher_algorithm, Style::default().fg(theme.text_main)),
        ]));

        let profile_status = if net.has_profile { "Saved" } else { "Not Saved" };
        right_lines.push(Line::from(vec![
            Span::styled("  Profile:     ", Style::default().fg(theme.text_dim)),
            Span::styled(profile_status, Style::default().fg(theme.text_main)),
        ]));

        right_lines.push(Line::from(""));
        right_lines.push(Line::from("  Signal Strength Gauge:"));
        right_lines.push(Line::from(""));

        f.render_widget(Paragraph::new(right_lines).block(right_block), area);

        let gauge_area = Rect {
            x: area.x + 2,
            y: area.y + 16,
            width: area.width.saturating_sub(4),
            height: 1,
        };
        let use_unicode = app.glyphs.status_ok == "✔️";
        let gauge = crate::ui::accent_gauge::AccentGauge::new(
            net.signal_quality as f64 / 100.0,
            "",
            theme.accent,
            theme.border,
            use_unicode,
            true,
        );
        f.render_widget(gauge, gauge_area);
    } else {
        right_lines.push(Line::from(""));
        right_lines.push(Line::from(Span::styled("  Select a network from the left to view details.", Style::default().fg(theme.text_dim))));
        f.render_widget(Paragraph::new(right_lines).block(right_block), area);
    }
}
