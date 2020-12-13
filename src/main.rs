use wordninja::DEFAULT_MODEL;
use std::env;

fn main() {
    let mut args = env::args();
    args.next();
    let input = args.next().unwrap();

    let words: Vec<_> = DEFAULT_MODEL.split(&input).collect();
    println!("{}", words.join(" "));
}