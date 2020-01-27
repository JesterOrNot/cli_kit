use std::string::ToString;

/// Take an item that implements ToString and return in red
///
/// Example
///
/// ```rust
///
/// use cli_kit::ansi::color_codes::red;
///
/// fn main() {
///     println!("{}", red(5));
/// }
/// ```
pub fn red<T: ToString>(text: T) -> String {
    "\x1b[31m".to_owned() + &text.to_string() + "\x1b[0m"
}

/// Take an item that implements ToString and return in red bold
///
/// Example
///
/// ```rust
///
/// use cli_kit::ansi::color_codes::red_bold;
///
/// fn main() {
///     println!("{}", red_bold(5));
/// }
/// ```
pub fn red_bold<T: ToString>(text: T) -> String {
    "\x1b[1;31m".to_owned() + &text.to_string() + "\x1b[0m"
}

/// Take an item that implements ToString and return in green
///
/// Example
///
/// ```rust
///
/// use cli_kit::ansi::color_codes::green;
///
/// fn main() {
///     println!("{}", green(5));
/// }
/// ```
pub fn green<T: ToString>(text: T) -> String {
    "\x1b[32m".to_owned() + &text.to_string() + "\x1b[0m"
}

/// Take an item that implements ToString and return in green bold
///
/// Example
///
/// ```rust
///
/// use cli_kit::ansi::color_codes::green_bold;
///
/// fn main() {
///     println!("{}", green_bold(5));
/// }
/// ```
pub fn green_bold<T: ToString>(text: T) -> String {
    "\x1b[1;32m".to_owned() + &text.to_string() + "\x1b[0m"
}
