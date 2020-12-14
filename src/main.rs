use std::{error::Error, fs};

use wordninja::{LanguageModel, DEFAULT_MODEL};

use clap::{App, Arg};

fn main() -> Result<(), Box<dyn Error>> {
    let opts = App::new("wordninja")
        .arg(
            Arg::with_name("n")
                .short("n")
                .help("do not output the trailing newline"),
        )
        .arg(
            Arg::with_name("seperator")
                .short("s")
                .long("sep")
                .help("separator between output words")
                .takes_value(true)
                .default_value(" "),
        )
        .arg(
            Arg::with_name("model")
                .short("m")
                .long("model")
                .help("Path to a custom model (must be uncompressed)")
                .takes_value(true)
                .value_name("FILE"),
        )
        .arg(Arg::with_name("INPUT").index(1).required(true))
        .get_matches();

    let input = opts.value_of("INPUT").unwrap();
    let sep = opts.value_of("seperator").unwrap();

    let words: Vec<_> = if let Some(model_path) = opts.value_of("model") {
        let model_str = fs::read_to_string(model_path)?;
        let model = LanguageModel::new_model(model_str.as_str());
        model.split(&input)
    } else {
        DEFAULT_MODEL.split(&input)
    };

    let result = words.join(sep);

    if opts.is_present("n") {
        print!("{}", result);
    } else {
        println!("{}", result);
    }

    Ok(())
}
