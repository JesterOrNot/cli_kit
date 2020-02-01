extern crate cli_kit;
extern crate rustyline;
use std::convert::From;
use std::iter::Peekable;

fn main() {
    let mut colored_version = String::new();
    let lexed_input = lex(&"2 + 3 - 65 + 3 ([]) ffff fff f".to_owned());
    for i in lexed_input.unwrap() {
        match i {
            LexItem::Num(n) => {
                colored_version += &cli_kit::ansi::color_codes::magenta(n, false, false).to_string()
            }
            LexItem::Op(n) => {
                colored_version += &cli_kit::ansi::color_codes::cyan(n, false, false).to_string()
            }
            LexItem::Paren(n) => {
                colored_version += &cli_kit::ansi::color_codes::green(n, false, false).to_string()
            }
            LexItem::Whitespace(_) | LexItem::Char(_) => colored_version += &String::from(i),
        };
    }
    println!("{}", colored_version)
}

impl From<LexItem> for String {
    fn from(item: LexItem) -> Self {
        match item {
            LexItem::Char(i) => i.to_string(),
            LexItem::Whitespace(i) => String::from(" "),
            _ => String::from(""),
        }
    }
}

#[derive(Debug, Clone)]
enum LexItem {
    Paren(char),
    Op(char),
    Char(char),
    Num(char),
    Whitespace(char),
}

fn lex(input: &String) -> Result<Vec<LexItem>, String> {
    let mut result = Vec::new();

    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            '0'...'9' => {
                it.next();
                result.push(LexItem::Num(c));
            }
            '+' | '*' | '-' | '/' => {
                result.push(LexItem::Op(c));
                it.next();
            }
            '(' | ')' | '[' | ']' | '{' | '}' => {
                result.push(LexItem::Paren(c));
                it.next();
            }
            ' ' => {
                result.push(LexItem::Whitespace(c));
                it.next();
            }
            _ => {
                result.push(LexItem::Char(c));
                it.next();
            }
        }
    }
    Ok(result)
}
