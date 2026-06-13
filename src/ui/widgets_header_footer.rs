use ratatui::{
    Frame,
    layout::{Rect, Margin},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use crate::app::AppState;
use crate::ui::ThemeColors;

pub fn draw_header(f: &mut Frame, app: &mut AppState, theme: &ThemeColors, area: Rect) {
    let username = &app.username;
    let hostname = &app.hostname;
    let os_version = app.os_version.clone();

    let header_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .title(Span::styled(" Rust Wi-Fi Manager ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)));

    let ver_str = format!(" scout v{} ", env!("CARGO_PKG_VERSION"));
    let user_host_str = format!("{}@{}", username, hostname);
    let os_str_val = os_version;

    let button_y = area.y + 1;
    let inner_width = area.width.saturating_sub(2) as usize;
    let left_len = ver_str.len() + 3 + user_host_str.len() + 3 + os_str_val.len();
    let right_len = 6 + 3 + 6;

    let header_line = if inner_width > left_len + right_len {
        let padding_len = inner_width - (left_len + right_len);
        let padding_str = " ".repeat(padding_len);

        let help_offset = 1 + left_len + padding_len;
        let help_start_x = area.x + help_offset as u16;
        let help_end_x = help_start_x + 6;
        app.help_btn_bounds = Some((button_y, help_start_x, help_end_x));

        let quit_offset = help_offset + 6 + 3;
        let quit_start_x = area.x + quit_offset as u16;
        let quit_end_x = quit_start_x + 6;
        app.quit_btn_bounds = Some((button_y, quit_start_x, quit_end_x));

        Line::from(vec![
            Span::styled(ver_str, Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
            Span::styled(" │ ", Style::default().fg(theme.border)),
            Span::styled(user_host_str, Style::default().fg(Color::Rgb(255, 215, 0)).add_modifier(Modifier::BOLD)),
            Span::styled(" │ ", Style::default().fg(theme.border)),
            Span::styled(os_str_val, Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
            Span::styled(padding_str, Style::default()),
            // Help button
            Span::styled(" ", Style::default().bg(Color::Rgb(250, 210, 50)).fg(Color::Black).add_modifier(Modifier::BOLD)),
            Span::styled("h", Style::default().bg(Color::Rgb(250, 210, 50)).fg(Color::Black).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)),
            Span::styled("elp ", Style::default().bg(Color::Rgb(250, 210, 50)).fg(Color::Black).add_modifier(Modifier::BOLD)),
            Span::styled(" │ ", Style::default().fg(theme.border)),
            // Quit button
            Span::styled(" ", Style::default().bg(Color::Rgb(255, 85, 85)).fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::styled("q", Style::default().bg(Color::Rgb(255, 85, 85)).fg(Color::White).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)),
            Span::styled("uit ", Style::default().bg(Color::Rgb(255, 85, 85)).fg(Color::White).add_modifier(Modifier::BOLD)),
        ])
    } else {
        let help_offset = 1 + ver_str.len() + 3 + user_host_str.len() + 3 + os_str_val.len() + 3;
        let help_start_x = area.x + help_offset as u16;
        let help_end_x = help_start_x + 6;
        app.help_btn_bounds = Some((button_y, help_start_x, help_end_x));

        let quit_offset = help_offset + 6 + 3;
        let quit_start_x = area.x + quit_offset as u16;
        let quit_end_x = quit_start_x + 6;
        app.quit_btn_bounds = Some((button_y, quit_start_x, quit_end_x));

        Line::from(vec![
            Span::styled(ver_str, Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
            Span::styled(" │ ", Style::default().fg(theme.border)),
            Span::styled(user_host_str, Style::default().fg(Color::Rgb(255, 215, 0)).add_modifier(Modifier::BOLD)),
            Span::styled(" │ ", Style::default().fg(theme.border)),
            Span::styled(os_str_val, Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
            Span::styled(" │ ", Style::default().fg(theme.border)),
            // Help button
            Span::styled(" ", Style::default().bg(Color::Rgb(250, 210, 50)).fg(Color::Black).add_modifier(Modifier::BOLD)),
            Span::styled("h", Style::default().bg(Color::Rgb(250, 210, 50)).fg(Color::Black).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)),
            Span::styled("elp ", Style::default().bg(Color::Rgb(250, 210, 50)).fg(Color::Black).add_modifier(Modifier::BOLD)),
            Span::styled(" │ ", Style::default().fg(theme.border)),
            // Quit button
            Span::styled(" ", Style::default().bg(Color::Rgb(255, 85, 85)).fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::styled("q", Style::default().bg(Color::Rgb(255, 85, 85)).fg(Color::White).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)),
            Span::styled("uit ", Style::default().bg(Color::Rgb(255, 85, 85)).fg(Color::White).add_modifier(Modifier::BOLD)),
        ])
    };

    f.render_widget(header_block, area);
    f.render_widget(Paragraph::new(header_line), area.inner(Margin { horizontal: 1, vertical: 1 }));
}

pub fn draw_footer(f: &mut Frame, app: &mut AppState, theme: &ThemeColors, area: Rect) {
    let footer_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .title(Span::styled(" Status ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)));

    let status_color = if app.status_msg.contains("failed") || app.status_msg.contains("failed:") || app.status_msg.contains("Code") {
        Color::Rgb(250, 80, 80)
    } else if app.status_msg.contains("Successfully") || app.status_msg.contains("Connected") {
        Color::Rgb(80, 250, 80)
    } else {
        theme.text_dim
    };

    let status_icon = if status_color == Color::Rgb(250, 80, 80) {
        app.glyphs.status_err
    } else if status_color == Color::Rgb(80, 250, 80) {
        app.glyphs.status_ok
    } else {
        app.glyphs.info
    };

    let footer_line = Line::from(vec![
        Span::styled(format!("{} ", status_icon), Style::default().fg(status_color)),
        Span::styled(&app.status_msg, Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
    ]);

    f.render_widget(footer_block, area);
    f.render_widget(Paragraph::new(footer_line), area.inner(Margin { horizontal: 1, vertical: 1 }));
}
