/// utf-normalize command-line interface.
//     Copyright (C) 2024  Dustin Thomas <io@cptlobster.dev>
//
//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU General Public License as published by
//     the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.
//
//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU General Public License for more details.
//
//     You should have received a copy of the GNU General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::io::{Read, Write};
use clap::Parser;
use clio::{Input, Output};
use libnormalize::translators::{translate, range_translation, multirange_translation, ascii_filter};

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

    let mathematical_an_translator = [
        // return first on ASCII chars
        ascii_filter(),
        // Mathematical bold, italic, bold/italic, script, bold script; uppercase
        multirange_translation('\u{1D400}', 'A', 26, 52, 5),
        // Mathematical bold, italic, bold/italic, script, bold script; lowercase
        multirange_translation('\u{1D41A}', 'a', 26, 52, 5),
        // Mathematical bold fraktur; sans, bold, italic, bold/italic uppercase
        multirange_translation('\u{1D56C}', 'A', 26, 52, 5),
        // Mathematical bold fraktur; sans, bold, italic, bold/italic lowercase
        multirange_translation('\u{1D586}', 'a', 26, 52, 5)
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