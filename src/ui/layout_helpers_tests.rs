use super::*;
use ratatui::layout::Rect;

#[test]
fn test_button_rect_new() {
    let button = ButtonRect::new(5, 10, 20);
    assert_eq!(button.y, 5);
    assert_eq!(button.x_start, 10);
    assert_eq!(button.x_end, 20);
}

#[test]
fn test_button_rect_contains() {
    let button = ButtonRect::new(5, 10, 20);

    // Inside
    assert!(button.contains(5, 10));
    assert!(button.contains(5, 15));
    assert!(button.contains(5, 19));

    // Outside (wrong y)
    assert!(!button.contains(4, 15));
    assert!(!button.contains(6, 15));

    // Outside (wrong x)
    assert!(!button.contains(5, 9));
    assert!(!button.contains(5, 20));
    assert!(!button.contains(5, 25));
}

#[test]
fn test_centered_rect() {
    let parent = Rect::new(0, 0, 100, 100);
    let centered = centered_rect(50, 50, parent);

    // For width 100 and percent 50, x should start around (100 - 50)/2 = 25
    // and width should be 50.
    assert!(centered.x > 0);
    assert!(centered.y > 0);
    assert!(centered.width > 0);
    assert!(centered.height > 0);

    // Let's assert centered rect is smaller than parent
    assert!(centered.width < parent.width);
    assert!(centered.height < parent.height);

    // Bounds check
    assert!(centered.x + centered.width <= parent.width);
    assert!(centered.y + centered.height <= parent.height);
}
