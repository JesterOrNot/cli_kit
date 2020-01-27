#[cfg(test)]
mod tests {
    use cli_kit::ansi::color_codes::*;
    #[test]
    fn test_red() {
        assert_eq!(red(5), "\x1b[31m5\x1b[0m");
    }
    #[test]
    fn test_red_bold() {
        assert_eq!(red_bold(5), "\x1b[1;31m5\x1b[0m");
    }
    #[test]
    fn test_green() {
        assert_eq!(green(5), "\x1b[32m5\x1b[0m");
    }
    #[test]
    fn test_green_bold() {
        assert_eq!(green_bold(5), "\x1b[1;32m5\x1b[0m");
    }
}
