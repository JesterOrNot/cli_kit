use std::string::ToString;

pub fn red<T: ToString>(text: T) -> String {
    "\x1b[31m".to_owned() + &text.to_string() + "\x1b[0m"
}

pub fn red_bold<T: ToString>(text: T) -> String {
    "\x1b[1;31m".to_owned() + &text.to_string() + "\x1b[0m"
}


pub fn green<T: ToString>(text: T) -> String {
    "\x1b[32m".to_owned() + &text.to_string() + "\x1b[0m"
}

pub fn green_bold<T: ToString>(text: T) -> String {
    "\x1b[1;32m".to_owned() + &text.to_string() + "\x1b[0m"
}
