#![allow(clippy::vec_init_then_push)]
//! Overlay popups and modal dialogues drawing functions.
//!
//! **Taxonomy Classification**: UI Rendering (UI Overlays).

use ratatui::{
    Frame,
    layout::Margin,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Clear},
};
use crate::app::AppState;
use crate::ui::ThemeColors;
use crate::ui::layout::{centered_rect, centered_rect_fixed, wrap_text};

pub fn draw_profiles_overlay(f: &mut Frame, app: &mut AppState, theme: &ThemeColors) {
    let area = centered_rect_fixed(70, 20, f.area());
    f.render_widget(Clear, area);
    
    let overlay_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.accent))
        .title(Span::styled(" Saved Offline Profiles (Known Networks) ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)));
    
    let inner_rect = overlay_block.inner(area);
    f.render_widget(overlay_block, area);
    
    let popup_chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            ratatui::layout::Constraint::Min(1),
            ratatui::layout::Constraint::Length(3),
        ])
        .split(inner_rect);
    
    let mut list_lines = Vec::new();
    if app.profiles_list.is_empty() {
        list_lines.push(Line::from(""));
        list_lines.push(Line::from(Span::styled("  No saved wireless profiles found on this system.", Style::default().fg(theme.text_dim))));
    } else {
        for (idx, (name, _)) in app.profiles_list.iter().enumerate() {
            let is_selected = idx == app.profiles_selected_idx;
            let bullet = if is_selected { app.glyphs.play } else { app.glyphs.play_empty };
            let bullet_style = if is_selected {
                Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.text_dim)
            };
            let name_style = if is_selected {
                Style::default().fg(theme.text_main).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.text_dim)
            };
            list_lines.push(Line::from(vec![
                Span::styled(bullet, bullet_style),
                Span::styled(name, name_style),
            ]));
        }
    }
    
    f.render_widget(Paragraph::new(list_lines), popup_chunks[0].inner(Margin { horizontal: 2, vertical: 1 }));
    
    let mut controls_lines = Vec::new();
    controls_lines.push(Line::from(vec![
        Span::styled("  Up/Down ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
        Span::styled("Navigate   ", Style::default().fg(theme.text_main)),
        Span::styled("d / Del ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
        Span::styled("Delete Profile   ", Style::default().fg(theme.text_main)),
        Span::styled("Esc / p ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
        Span::styled("Close Manager", Style::default().fg(theme.text_main)),
    ]));
    f.render_widget(Paragraph::new(controls_lines), popup_chunks[1].inner(Margin { horizontal: 1, vertical: 1 }));
}

pub fn draw_help_overlay(f: &mut Frame, _app: &mut AppState, theme: &ThemeColors) {
    let area = centered_rect_fixed(70, 25, f.area());
    f.render_widget(Clear, area);

    let help_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.accent))
        .title(Span::styled(" scout Keyboard Shortcuts ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)));

    let mut help_lines = Vec::new();
    help_lines.push(Line::from(""));
    
    let mut add_shortcut = |key: &'static str, desc: &'static str| {
        let prefix_width = 17;
        let max_desc_width = (area.width as usize).saturating_sub(prefix_width + 4);
        let desc_wrapped = wrap_text(desc, max_desc_width);

        for (i, line) in desc_wrapped.iter().enumerate() {
            if i == 0 {
                help_lines.push(Line::from(vec![
                    Span::styled(format!("  {:<12}", key), Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
                    Span::styled(" │ ", Style::default().fg(theme.border)),
                    Span::styled(line.clone(), Style::default().fg(theme.text_main)),
                ]));
            } else {
                help_lines.push(Line::from(vec![
                    Span::styled("              ", Style::default().fg(theme.accent)),
                    Span::styled(" │ ", Style::default().fg(theme.border)),
                    Span::styled(line.clone(), Style::default().fg(theme.text_main)),
                ]));
            }
        }
    };

    add_shortcut("Tab", "Cycle focus between Station List and Info Panel");
    add_shortcut("Up / Down", "Navigate Wi-Fi networks");
    add_shortcut("k / j", "Navigate Wi-Fi networks (vim keys)");
    add_shortcut("Enter", "Connect to network (asks password if unsecured/no profile)");
    add_shortcut("d / Del", "Disconnect from active station or delete saved profile");
    add_shortcut("Space / r", "Force scan & refresh Wi-Fi networks list");
    add_shortcut("/", "Enter SSID filter/search mode");
    add_shortcut("t / T", "Toggle Wi-Fi radio hardware state (On/Off)");
    add_shortcut("s / S", "Share selected network credentials via QR Code");
    add_shortcut("p / P", "Manage saved offline profiles (known networks)");
    add_shortcut("h", "Toggle keyboard shortcuts overlay modal");
    add_shortcut("w / W", "Connect to a hidden network");
    add_shortcut("F1", "View README.md documentation");
    add_shortcut("F2", "View SUPPORT.md documentation");
    add_shortcut("F3", "View LICENSE.md documentation");
    add_shortcut("F4", "View COPYRIGHT.md documentation");
    add_shortcut("F5", "View PRIVACY.md documentation");
    add_shortcut("F6", "View SECURITY.md documentation");
    add_shortcut("F7", "View CONTRIBUTING.md documentation");
    add_shortcut("Esc / q", "Close active popups / Quit application");

    help_lines.push(Line::from(""));
    help_lines.push(Line::from(Span::styled("  Press Esc / q / h to close this help window.", Style::default().fg(theme.text_dim))));

    f.render_widget(Paragraph::new(help_lines).block(help_block).wrap(ratatui::widgets::Wrap { trim: false }), area);
}

pub fn draw_markdown_overlay(f: &mut Frame, app: &mut AppState, theme: &ThemeColors) {
    if let Some(ref filename) = app.show_markdown {
        let area = centered_rect(80, 80, f.area());
        f.render_widget(Clear, area);

        let doc_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.accent))
            .title(Span::styled(format!(" Documentation: {} (Press Esc/q to Close) ", filename), Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)));

        let paragraph = Paragraph::new(app.markdown_lines.clone())
            .block(doc_block)
            .wrap(ratatui::widgets::Wrap { trim: true })
            .alignment(ratatui::layout::Alignment::Left)
            .scroll((app.markdown_scroll as u16, 0));

        f.render_widget(paragraph, area);
    }
}
