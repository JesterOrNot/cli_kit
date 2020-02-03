/// Hides the cursor by printing ANSI escape sequence
pub fn hide_cursor() {
    print!("\x1b[?25l");
}
/// Shows the cursor by printing ANSI escape sequence
pub fn show_cursor() {
    print!("\x1b[?25h")
}

/// Clears the screen by printing ANSI escapse sequence
///
/// Paramaters
///
/// degree - how much is the screen cleared
///
/// 0 = clear from cursor to the end
///
/// 1 = clear from cursor to the start
///
/// 2 = clear the entire screen
///
/// Example
/// ```
/// use cli_kit::ansi::cursor::clear_screen;
/// fn main() {
///     clear_screen(2); // clears the entire screen
/// }
/// ```
pub fn clear_screen(degree: i32) {
    print!("\x1b[{}K", degree)
}
