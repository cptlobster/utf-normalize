mod homographs;

use std::io::{Read, Write};
use clap::Parser;
use clio::{Input, Output};
use homographs::{translate, range_translation};

/// Program for decoding Unicode homographs into their normal characters
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input data. Defaults to stdin.
    #[arg(short, long, value_parser, default_value="-")]
    input_file: Input,

    #[arg(short, long, value_parser, default_value="-")]
    output_file: Output,
}

fn main() {
    let mut args = Args::parse();

    let test_translator = [
        range_translation('a', 'A', 26),
        range_translation('A', 'a', 26)
    ];

    if (args.input_file.is_std()) {
        todo!("implement reading from stdin");
    }
    else {
        let f: &mut std::fs::File = args.input_file.get_file().unwrap();
        let mut res0: String = String::new();
        f.read_to_string(&mut res0).unwrap();
        let charray = res0.chars();
        let res1 = charray.map(|a| translate(a, &test_translator)).collect::<String>();
        args.output_file.write(res1.as_bytes()).unwrap();
    }
}