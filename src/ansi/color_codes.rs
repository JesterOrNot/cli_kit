pub fn Red<T: std::string::ToString>(text: T) -> String {
    
    return "\x1b[31m".to_owned() + &text.to_string() + "\x1b[0m"
}