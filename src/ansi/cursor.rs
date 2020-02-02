/// Hides the cursor by printing ANSI escape sequence
pub fn hide_cursor() {
    print!("\x1b[?25l");
}
/// Shows the cursor by printing ANSI escape sequence
pub fn show_cursor() {
    print!("\x1b[?25h")
}