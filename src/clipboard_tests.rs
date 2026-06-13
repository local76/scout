use super::*;

#[test]
fn test_copy_text_to_clipboard() {
    let result = copy_text_to_clipboard("scout_test_clipboard_content");
    match result {
        Ok(()) => {
            // Success - copy succeeded (e.g. if wl-copy, xclip, or xsel was available and succeeded)
        }
        Err(e) => {
            // Expected failure if no clipboard utility is installed or accessible in this environment
            assert_eq!(e.kind(), std::io::ErrorKind::NotFound);
        }
    }
}
