use cli_kit::ansi::color_codes::{blue, green, red, yellow};

fn main() {
    println!(
        "{} {} {} {} {} {} {} {} {}",
        red(5, false, false),
        red(5, false, true),
        red(5, true, false),
        green(5, false),
        green(5, true),
        yellow(5, false),
        yellow(5, true),
        blue(5, false),
        blue(5, true)
    );
    println!("{}", green("Hello World From Rust!", true))
}
