use super::*;
use ratatui::style::Color;

#[test]
fn test_accent_colors_new() {
    let colors = AccentColors::new(Color::Red, Color::Blue, Color::Green);
    assert_eq!(colors.accent, Color::Red);
    assert_eq!(colors.dim, Color::Blue);
    assert_eq!(colors.text, Color::Green);

    let colors2 = AccentColors::from_accent_dim_text(Color::Yellow, Color::Cyan, Color::Magenta);
    assert_eq!(colors2.accent, Color::Yellow);
    assert_eq!(colors2.dim, Color::Cyan);
    assert_eq!(colors2.text, Color::Magenta);
}

#[test]
fn test_accent_colors_query_system() {
    let colors = AccentColors::query_system();
    // System query should yield some colors, check they are not uninitialized
    // (accent will map to the mock/queried color)
    assert_ne!(colors.accent, Color::Reset);
}

#[test]
fn test_calculate_from_accent() {
    // Test dark mode calculation
    let dark_cyan = AccentColors::calculate_from_accent(Color::Cyan, true);
    assert_eq!(dark_cyan.accent, Color::Cyan);
    assert_eq!(dark_cyan.text, Color::Gray);
    match dark_cyan.dim {
        Color::Rgb(r, g, b) => {
            // Cyan: (0, 245, 255)
            // dim = (0 * 0.35, 245 * 0.35, 255 * 0.35) = (0, 85, 89)
            assert_eq!(r, 0);
            assert_eq!(g, 85);
            assert_eq!(b, 89);
        }
        _ => panic!("Expected RGB dim color"),
    }

    // Test light mode calculation
    let light_red = AccentColors::calculate_from_accent(Color::Red, false);
    assert_eq!(light_red.accent, Color::Red);
    assert_eq!(light_red.text, Color::Black);
    match light_red.dim {
        Color::Rgb(r, g, b) => {
            // Red: (255, 0, 0)
            // dim = (255 * 0.7, 0, 0) = (178, 0, 0)
            assert_eq!(r, 178);
            assert_eq!(g, 0);
            assert_eq!(b, 0);
        }
        _ => panic!("Expected RGB dim color"),
    }
}

#[test]
fn test_accent_theme_fallbacks() {
    let dark = AccentTheme::default_dark();
    assert_eq!(dark.accent, Color::Rgb(0, 245, 255));
    assert_eq!(dark.dim, Color::Rgb(0, 80, 85));
    assert_eq!(dark.text, Color::Gray);

    let light = AccentTheme::default_light();
    assert_eq!(light.accent, Color::Rgb(0, 180, 200));
    assert_eq!(light.dim, Color::Rgb(180, 230, 240));
    assert_eq!(light.text, Color::Black);

    // Assert AccentTheme::current does not panic
    let _current = AccentTheme::current();
}
