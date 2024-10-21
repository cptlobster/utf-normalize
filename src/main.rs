mod homographs;

use std::io::{Read, Write};
use clap::Parser;
use clio::{Input, Output};
use homographs::{translate, range_translation};

/// Program for normalizing uncommon Unicode characters into their ASCII equivalents.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input data. Defaults to stdin.
    #[arg(short, long, value_parser, default_value="-")]
    input_file: Input,

    /// Location to output to. Defaults to stdout.
    #[arg(short, long, value_parser, default_value="-")]
    output_file: Output,
}

fn main() {
    let mut args = Args::parse();

    /*
     * This is just a test translator; it converts ASCII characters from lowercase to uppercase, and
     * vice versa.
     */
    let test_translator = [
        range_translation('a', 'A', 26),
        range_translation('A', 'a', 26)
    ];

    /*
     * This is another test translator that implements the Caesar cipher (with a right rotation of
     * 1).
     */
    let caesar_translator = [
        range_translation('a', 'b', 25),
        range_translation('z', 'a', 1),
        range_translation('A', 'B', 25),
        range_translation('Z', 'A', 1)
    ];

    /* Read input (for reading from stdin, this is intended to be a pipe) */
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