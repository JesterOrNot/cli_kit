pub fn Red(text: &str) -> String {
    return "\x1b[31m".to_owned() + text + "\x1b[0m"
}