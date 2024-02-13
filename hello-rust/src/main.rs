use std::io::{stdout, BufWriter};
use ferris_says::say;
mod calc;

fn main() {
    let stdout = stdout();
    let message = String::from("Hello Fellow Rustceasn!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();

    calc::sum();
}
