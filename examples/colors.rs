use cli_kit::ansi::color_codes::*;

fn main() {
    println!("{} {} {} {}", red(5), red_bold(5), green(5), green_bold(5));
    println!("{}", green("Hello World From Rust!"))
}
