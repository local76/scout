//! Layout and positioning utility helpers for ratatui TUIs.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
};
use crate::ui::theme::ThemeColors;
use crate::ui::text::wrap_text;

/// Represents the boundary coordinates of a clickable button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ButtonRect {
    pub y: u16,
    pub x_start: u16,
    pub x_end: u16,
}

impl ButtonRect {
    pub fn new(y: u16, x_start: u16, x_end: u16) -> Self {
        Self { y, x_start, x_end }
    }

    /// Checks if a mouse event coordinate falls inside the button boundary.
    pub fn contains(&self, mouse_row: u16, mouse_col: u16) -> bool {
        mouse_row == self.y && mouse_col >= self.x_start && mouse_col < self.x_end
    }
}


/// Center a rect of specified percentage width and height inside another rect.
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

#[cfg(test)]
#[path = "layout_helpers_tests.rs"]
mod tests;


