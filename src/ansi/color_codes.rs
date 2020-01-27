use std::string::ToString;

pub fn red<T: ToString>(text: T) -> String {
    "\x1b[31m".to_owned() + &text.to_string() + "\x1b[0m"
}
