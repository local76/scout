use std::time::{Duration, Instant};
use ratatui::{backend::TestBackend, Terminal};
use crate::app::AppState;
use crate::ui::draw_ui;
use crate::ui::theme::get_theme;

#[test]
fn test_ui_rendering_perf_budget() {
    let mut app = AppState::new();
    let theme = get_theme(true, ratatui::style::Color::Rgb(0, 245, 255));
    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).expect("Failed to create test terminal");
    
    // Warmup
    terminal.draw(|f| draw_ui(f, &mut app, &theme)).unwrap();
    
    // Benchmark 100 frames
    const FRAMES: usize = 100;
    let start = Instant::now();
    for _ in 0..FRAMES {
        terminal.draw(|f| draw_ui(f, &mut app, &theme)).unwrap();
    }
    let elapsed = start.elapsed();
    
    let budget = Duration::from_millis(3000);
    assert!(
        elapsed < budget,
        "100 frames took {:?}, exceeding budget of {:?}",
        elapsed,
        budget
    );
    println!("TUI Render Loop Performance: {} frames in {:?}", FRAMES, elapsed);
}
