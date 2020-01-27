use cli_kit::ansi::color_codes::{green, green_bold, red, red_bold};

fn main() {
    println!("{} {} {} {}", red(5), red_bold(5), green(5), green_bold(5));
    println!("{}", green_bold("Hello World From Rust!"))
}
