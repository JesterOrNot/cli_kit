use cli_kit;
fn main() {
    println!("{} {}", cli_kit::ansi::color_codes::red(5),cli_kit::ansi::color_codes::red_bold(5));
}