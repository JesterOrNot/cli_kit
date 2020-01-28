#[cfg(test)]
mod tests {
    use cli_kit::ansi::color_codes::{blue, green, red, yellow};
    #[test]
    fn test_red() {
        assert_eq!(red(5, false, false), "\x1b[31m5\x1b[0m");
    }
    #[test]
    fn test_red_bold() {
        assert_eq!(red(5, true, false), "\x1b[1m\x1b[31m5\x1b[0m");
    }
    #[test]
    fn test_red_underline() {
        assert_eq!(red(5, false, true), "\x1b[4m\x1b[31m5\x1b[0m");
    }
    #[test]
    fn test_green() {
        assert_eq!(green(5, false, false), "\x1b[32m5\x1b[0m");
    }
    #[test]
    fn test_green_bold() {
        assert_eq!(green(5, true, false), "\x1b[1m\x1b[32m5\x1b[0m");
    }
    #[test]
    fn test_green_underline() {
        assert_eq!(green(5, false, true), "\x1b[4m\x1b[32m5\x1b[0m")
    }
    #[test]
    fn test_yellow() {
        assert_eq!(yellow(5, false), "\x1b[33m5\x1b[0m");
    }
    #[test]
    fn test_yellow_bold() {
        assert_eq!(yellow(5, true), "\x1b[1m\x1b[33m5\x1b[0m");
    }
    #[test]
    fn test_blue() {
        assert_eq!(blue(5, false), "\x1b[34m5\x1b[0m");
    }
    #[test]
    fn test_blue_bold() {
        assert_eq!(blue(5, true), "\x1b[1m\x1b[34m5\x1b[0m");
    }
}
