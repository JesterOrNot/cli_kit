#[cfg(test)]
mod tests {
    use cli_kit::ansi::color_codes::red;
    #[test]
    fn test_red_bold() {
        assert_eq!(red(5), "\x1b[31m5\x1b[0m");
    }
}