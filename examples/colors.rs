use cli_kit::ansi::color_codes::{
    blue, blue_bold, green, red, yellow,
};

fn main() {
    println!(
        "{} {} {} {} {} {} {} {}",
        red(5, false),
        red(5, true),
        green(5, false),
        green(5, true),
        yellow(5, false),
        yellow(5, true),
        blue(5),
        blue_bold(5)
    );
    println!("{}", green("Hello World From Rust!", true))
}
