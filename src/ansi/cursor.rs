pub fn hide_cursor() {
    println!("\x1b[?25l");
}
pub fn show_cursor() {
    println!("\x1b[?25h")
}