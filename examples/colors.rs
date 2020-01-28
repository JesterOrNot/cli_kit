use cli_kit::ansi::color_codes::{blue, green, magenta, red, yellow};

fn main() {
    println!(
        "{} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
        red(5, false, false),
        red(5, false, true),
        red(5, true, false),
        green(5, false, false),
        green(5, true, false),
        green(5, false, true),
        yellow(5, false, false),
        yellow(5, true, false),
        yellow(5, false, true),
        blue(5, false, false),
        blue(5, true, false),
        blue(5, false, true),
        magenta(5, false, false),
        magenta(5, true, false),
        magenta(5, false, true)
    );
    println!("{}", green("Hello World From Rust!", true, true))
}
