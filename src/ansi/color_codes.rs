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
///     //underline
///     println!("{}", red(5, false, true));
///     // bold
///     println!("{}", red(5, true, false));
///     // normal
///     println!("{}", red(5, false, false));
/// }
/// ```
pub fn red<T: ToString>(text: T, bold: bool, underline: bool) -> String {
    let mut result = String::from("\x1b[31m".to_owned() + &text.to_string() + "\x1b[0m");
    if bold {
        result = "\x1b[1m".to_owned() + &result;
    }
    if underline {
        result = "\x1b[4m".to_owned() + &result;
    }
    return result;
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
///     // underline
///     println!("{}", green(5, false, true));
///     // bold
///     println!("{}", green(5, true, false));
///     // normal
///     println!("{}", green(5, false, false));
/// }
/// ```
pub fn green<T: ToString>(text: T, bold: bool, underline: bool) -> String {
    let mut result = String::from("\x1b[32m".to_owned() + &text.to_string() + "\x1b[0m");
    if bold {
        result = "\x1b[1m".to_owned() + &result;
    }
    if underline {
        result = "\x1b[4m".to_owned() + &result;
    }
    return result;
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
///     // underline
///     println!("{}", yellow(5, false, true));
///     // bold
///     println!("{}", yellow(5, true, false));
///     // normal
///     println!("{}", yellow(5, false, false));
/// }
/// ```
pub fn yellow<T: ToString>(text: T, bold: bool, underline: bool) -> String {
    let mut result = String::from("\x1b[33m".to_owned() + &text.to_string() + "\x1b[0m");
    if bold {
        result = "\x1b[1m".to_owned() + &result;
    }
    if underline {
        result = "\x1b[4m".to_owned() + &result;
    }
    return result;
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
///     // underline
///     println!("{}", blue(5, false, true));
///     // bold
///     println!("{}", blue(5, true, false));
///     // normal
///     println!("{}", blue(5, false, false))
/// }
/// ```
pub fn blue<T: ToString>(text: T, bold: bool, underline: bool) -> String {
    let mut result = String::from("\x1b[34m".to_owned() + &text.to_string() + "\x1b[0m");
    if bold {
        result = "\x1b[1m".to_owned() + &result;
    }
    if underline {
        result = "\x1b[4m".to_owned() + &result;
    }
    return result;
}
/// Take an item that implements ToString and return in magenta
///
/// Example
///
/// ```rust
///
/// use cli_kit::ansi::color_codes::magenta;
///
/// fn main() {
///     // underline
///     println!("{}", magenta(5, false, true));
///     // bold
///     println!("{}", magenta(5, true, false));
///     // normal
///     println!("{}", magenta(5, false, false))
/// }
/// ```
pub fn magenta<T: ToString>(text: T, bold: bool, underline: bool) -> String {
    let mut result = String::from("\x1b[35m".to_owned() + &text.to_string() + "\x1b[0m");
    if bold {
        result = "\x1b[1m".to_owned() + &result;
    }
    if underline {
        result = "\x1b[4m".to_owned() + &result;
    }
    return result;
}
//// Take an item that implements ToString and return in cyan
///
/// Example
///
/// ```rust
///
/// use cli_kit::ansi::color_codes::cyan;
///
/// fn main() {
///     // underline
///     println!("{}", cyan(5, false, true));
///     // bold
///     println!("{}", cyan(5, true, false));
///     // normal
///     println!("{}", cyan(5, false, false))
/// }
/// ```
pub fn cyan<T: ToString>(text: T, bold: bool, underline: bool) -> String {
    let mut result = String::from("\x1b[36m".to_owned() + &text.to_string() + "\x1b[0m");
    if bold {
        result = "\x1b[1m".to_owned() + &result;
    }
    if underline {
        result = "\x1b[4m".to_owned() + &result;
    }
    return result;
}
