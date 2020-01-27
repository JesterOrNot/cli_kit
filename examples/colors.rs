use cli_kit::ansi::color_codes::{
    blue, blue_bold, green, green_bold, red, red_bold, yellow, yellow_bold,
};

fn main() {
    println!(
        "{} {} {} {} {} {} {} {}",
        red(5),
        red_bold(5),
        green(5),
        green_bold(5),
        yellow(5),
        yellow_bold(5),
        blue(5),
        blue_bold(5)
    );
    println!("{}", green_bold("Hello World From Rust!"))
}
