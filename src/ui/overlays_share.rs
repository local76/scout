use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Margin},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Clear},
};
use crate::app::AppState;
use crate::ui::ThemeColors;
use crate::ui::layout::centered_rect_fixed;

pub fn draw_share_overlay(f: &mut Frame, app: &mut AppState, theme: &ThemeColors) {
    let area = centered_rect_fixed(50, 24, f.area());
    f.render_widget(Clear, area);
    
    let overlay_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.accent))
        .title(Span::styled(" Share Wi-Fi Network ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)));
    
    let inner_rect = overlay_block.inner(area);
    f.render_widget(overlay_block, area);
    
    if app.share_prompt_password {
        let popup_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .split(inner_rect);
        
        let mut overlay_lines = Vec::new();
        overlay_lines.push(Line::from(vec![
            Span::styled("  SSID: ", Style::default().fg(theme.text_dim)),
            Span::styled(&app.share_ssid, Style::default().fg(theme.text_main).add_modifier(Modifier::BOLD)),
        ]));
        f.render_widget(Paragraph::new(overlay_lines), popup_chunks[0]);
        
        let input_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border_active))
            .title(Span::styled(" Enter Password to Generate QR Code ", Style::default().fg(theme.accent)));
        
        let display_pwd = app.share_box.text.clone();
        let cursor_str = if app.share_box.cursor_pos == app.share_box.text.len() { "_" } else { "" };
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
            Span::styled("Generate   ", Style::default().fg(theme.text_main)),
            Span::styled("Esc ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
            Span::styled("Cancel", Style::default().fg(theme.text_main)),
        ]));
        f.render_widget(Paragraph::new(controls_lines), popup_chunks[2]);
    } else {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Min(10),
                Constraint::Length(2),
            ])
            .split(inner_rect);
        
        f.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled("  SSID: ", Style::default().fg(theme.text_dim)),
                Span::styled(&app.share_ssid, Style::default().fg(theme.text_main).add_modifier(Modifier::BOLD)),
            ])).alignment(ratatui::layout::Alignment::Center),
            chunks[0],
        );
        
        let mut qr_lines = Vec::new();
        for line in &app.share_qr_lines {
            qr_lines.push(Line::from(Span::styled(line.clone(), Style::default().bg(Color::White).fg(Color::Black))));
        }
        
        f.render_widget(
            Paragraph::new(qr_lines).alignment(ratatui::layout::Alignment::Center),
            chunks[1],
        );
        
        f.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled("Press Esc / s / q to close this popup", Style::default().fg(theme.text_dim)),
            ])).alignment(ratatui::layout::Alignment::Center),
            chunks[2],
        );
        
    }
}

pub fn draw_hidden_overlay(f: &mut Frame, app: &mut AppState, theme: &ThemeColors) {
    let area = centered_rect_fixed(60, 12, f.area());
    f.render_widget(Clear, area);
    
    let overlay_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.accent))
        .title(Span::styled(" Connect to Hidden Wi-Fi ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)));
    
    let inner_rect = overlay_block.inner(area);
    f.render_widget(overlay_block, area);
    
    let popup_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(2),
        ])
        .split(inner_rect);
    
    let (step_title, input_title, mask_char) = match app.hidden_prompt_step {
        1 => ("  Step 1: Specify Network SSID Name", " Network SSID Name ", false),
        2 => ("  Step 2: Is the network secured? (y/n)", " Secured? [y = Yes, n = No] (Default: y) ", false),
        3 => ("  Step 3: Specify Network Password", " Network Password ", true),
        _ => ("", "", false),
    };
    
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(step_title, Style::default().fg(theme.text_main).add_modifier(Modifier::BOLD)),
        ])),
        popup_chunks[0].inner(Margin { horizontal: 1, vertical: 1 }),
    );
    
    let input_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border_active))
        .title(Span::styled(input_title, Style::default().fg(theme.accent)));
    
    let display_val = if mask_char {
        "*".repeat(app.hidden_box.text.len())
    } else {
        app.hidden_box.text.clone()
    };
    let cursor_str = if app.hidden_box.cursor_pos == app.hidden_box.text.len() { "_" } else { "" };
    
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(format!(" {}{}", display_val, cursor_str), Style::default().fg(theme.text_main)),
        ])).block(input_block),
        popup_chunks[1].inner(Margin { horizontal: 2, vertical: 0 }),
    );
    
    let mut controls_lines = Vec::new();
    controls_lines.push(Line::from(vec![
        Span::styled("  Enter ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
        Span::styled("Next / Submit   ", Style::default().fg(theme.text_main)),
        Span::styled("Esc ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
        Span::styled("Cancel", Style::default().fg(theme.text_main)),
    ]));
    f.render_widget(Paragraph::new(controls_lines), popup_chunks[2]);
}
