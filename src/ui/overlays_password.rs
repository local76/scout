use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Margin},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Clear},
};
use crate::app::AppState;
use crate::ui::ThemeColors;
use crate::ui::layout::centered_rect_fixed;
use crate::win32;

pub fn draw_password_overlay(f: &mut Frame, app: &mut AppState, theme: &ThemeColors) {
    let filtered_nets: Vec<win32::WlanNetwork> = if app.search_active {
        app.networks
            .iter()
            .filter(|n| n.ssid.to_lowercase().contains(&app.search_box.text.to_lowercase()))
            .cloned()
            .collect()
    } else {
        app.networks.clone()
    };

    if let Some(net) = filtered_nets.get(app.selected_network_idx) {
        let area = centered_rect_fixed(60, 13, f.area());
        f.render_widget(Clear, area);
        
        let overlay_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.accent))
            .title(Span::styled(" Wi-Fi Password Required ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)));
        
        let inner_rect = overlay_block.inner(area);
        f.render_widget(overlay_block, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // 0: Details (SSID, Security)
                Constraint::Length(3), // 1: Input text box
                Constraint::Length(3), // 2: Operations controls text
            ])
            .split(inner_rect);

        let mut overlay_lines = Vec::new();
        overlay_lines.push(Line::from(vec![
            Span::styled("  SSID:     ", Style::default().fg(theme.text_dim)),
            Span::styled(&net.ssid, Style::default().fg(theme.text_main).add_modifier(Modifier::BOLD)),
        ]));
        overlay_lines.push(Line::from(vec![
            Span::styled("  Security: ", Style::default().fg(theme.text_dim)),
            Span::styled(&net.auth_algorithm, Style::default().fg(theme.text_main)),
        ]));
        f.render_widget(Paragraph::new(overlay_lines), popup_chunks[0]);

        let is_enterprise = net.auth_algorithm.contains("Enterprise") || net.auth_algorithm.contains("RSNA");
        let input_title = if is_enterprise && app.eap_prompt_username {
            " Enter EAP Username "
        } else if net.has_profile {
            " Enter Password (leave empty to use saved profile) "
        } else {
            " Enter Password "
        };
        let input_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border_active))
            .title(Span::styled(input_title, Style::default().fg(theme.accent)));
        
        let display_pwd = if app.password_visible {
            app.password_box.text.clone()
        } else {
            "*".repeat(app.password_box.text.len())
        };

        let cursor_str = if app.password_box.cursor_pos == app.password_box.text.len() {
            "_"
        } else {
            ""
        };

        f.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled(format!(" {}{}", display_pwd, cursor_str), Style::default().fg(theme.text_main)),
            ])).block(input_block),
            popup_chunks[1].inner(Margin { horizontal: 2, vertical: 0 }),
        );

        let mut controls_lines = Vec::new();
        controls_lines.push(Line::from(""));
        controls_lines.push(Line::from(vec![
            Span::styled("  Enter ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
            Span::styled("Connect   ", Style::default().fg(theme.text_main)),
            Span::styled("Tab ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
            Span::styled("Toggle visibility   ", Style::default().fg(theme.text_main)),
            Span::styled("Esc ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
            Span::styled("Cancel", Style::default().fg(theme.text_main)),
        ]));
        f.render_widget(Paragraph::new(controls_lines), popup_chunks[2]);
    }
}
