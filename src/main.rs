use wordninja::DEFAULT_MODEL;
use std::env;

use clap::{App, Arg};

fn main() {
    let opts = App::new("wordninja")
        .arg(Arg::with_name("newline")
             .long("newline")
             .short("n")
             .help("Print newline at the end")
             .takes_value(true)
             .possible_values(&["true", "false"])
             .default_value("true"))
        .arg(Arg::with_name("INPUT")
             .index(1)
             .required(true))
        .get_matches_from(env::args());

    let input = opts.value_of("INPUT").unwrap();
    let newline = opts.value_of("newline").unwrap_or("true");


    let words: Vec<_> = DEFAULT_MODEL.split(&input).collect();
    let result = words.join(" ");

    if newline == "true" {
      println!("{}", result);
    } else {
      print!("{}", result);
    }
}