pub use std::string::ToString;

/// Take an item that implements ToString and return in red
/// The second paramater is if it is bold true for bold false for nomal
///
/// Example
///
/// ```rust
///
/// use cli_kit::ansi::color_codes::red;
///
/// fn main() {
///     // bold
///     println!("{}", red(5, true));
///     // normal
///     println!("{}", red(5, false));
/// }
/// ```
pub fn red<T: ToString>(text: T, bold: bool) -> String {
    let mut result = String::from("\x1b[31m".to_owned() + &text.to_string() + "\x1b[0m");
    if bold {
        result = "\x1b[1m".to_owned() + &result;
    }
    return result
}

/// Take an item that implements ToString and return in green
/// The second paramater is if it is bold true for bold false for nomal
///
/// Example
///
/// ```rust
///
/// use cli_kit::ansi::color_codes::green;
///
/// fn main() {
///     // bold
///     println!("{}", green(5, true));
///     // normal
///     println!("{}", green(5, false));
/// }
/// ```
pub fn green<T: ToString>(text: T, bold: bool) -> String {
    let mut result = String::from("\x1b[32m".to_owned() + &text.to_string() + "\x1b[0m");
    if bold {
        result = "\x1b[1m".to_owned() + &result;
    }
    return result
}

/// Take an item that implements ToString and return in yellow
/// The second paramater is if it is bold true for bold false for nomal
///
/// Example
///
/// ```rust
///
/// use cli_kit::ansi::color_codes::yellow;
///
/// fn main() {
///     // bold
///     println!("{}", yellow(5, true));
///     // normal
///     println!("{}", yellow(5, false));
/// }
/// ```
pub fn yellow<T: ToString>(text: T, bold: bool) -> String {
    let mut result = String::from("\x1b[33m".to_owned() + &text.to_string() + "\x1b[0m");
    if bold {
        result = "\x1b[1m".to_owned() + &result;
    }
    return result
}

/// Take an item that implements ToString and return in blue
///
/// Example
///
/// ```rust
///
/// use cli_kit::ansi::color_codes::blue;
///
/// fn main() {
///     println!("{}", blue(5));
/// }
/// ```
pub fn blue<T: ToString>(text: T) -> String {
    "\x1b[34m".to_owned() + &text.to_string() + "\x1b[0m"
}

/// Take an item that implements ToString and return in blue bold
///
/// Example
///
/// ```rust
///
/// use cli_kit::ansi::color_codes::blue_bold;
///
/// fn main() {
///     println!("{}", blue_bold(5));
/// }
/// ```
pub fn blue_bold<T: ToString>(text: T) -> String {
    "\x1b[1;34m".to_owned() + &text.to_string() + "\x1b[0m"
}
