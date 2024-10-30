/// Configuration handling and translator generation.
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

use toml::Table;
use std::fs;
use regex::Regex;
use toml::Value::Boolean;
use crate::translators::{Translator, ascii_filter, range_translation, multirange_translation,
                         lookup_translation};

/// Parses a configuration file.
/// ## Format
/// Configuration files are written in TOML format. Each translator is defined as its own section,
/// in a format like so:
/// ```toml
/// # Sample translator to make any lowercase letters uppercase and vice versa.
/// [translator_1] # The section can be anything. Just make sure that it isn't "global".
/// type = "range" # Can be one of either "lookup", "range", or "multirange"
/// source = 'A' # See the translators module for each translator's config values.
/// target = 'a'
/// size = 26
///
/// [translator_2]
/// type = "range"
/// source = 'a'
/// target = 'A'
/// size = 26
/// ```
/// ## Global Fields
/// The "global" section handles global configs.
/// ```toml
/// [global]
/// use_ascii_filter = false # Enables the ASCII character filter
/// ```
/// ### Options
/// - `use_ascii_filter: boolean`: Determines whether [`ascii_filter`] will be applied.
fn parse(path: String) {
    let data: String = fs::read_to_string(path).unwrap();
    let mut config: Table = toml::from_str(&data).unwrap();

    let mut translators: Vec<Translator> = Vec::new();

    // deal with the default config parameters
    let use_af: bool = config.get("global.use_ascii_filter").unwrap_or(&Boolean(false)).as_bool()
        .unwrap_or(false);

    if use_af { translators.push(ascii_filter()); }

    config.keys().for_each(|section| {
        if (section != "global") {
            let t_type: String = config.get(section + ".type").unwrap().as_str().unwrap().to_string();

            match t_type.as_str() {
                "range" => {
                    let src_str = config.get(section + ".source").unwrap().as_str().unwrap();
                    let trg_str = config.get(section + ".target").unwrap().as_str().unwrap();
                    let source: Option<char> = getchar(src_str, section);
                    let target: Option<char> = getchar(trg_str, section);
                    let size: u32 = config.get(section + ".size").unwrap().as_int().unwrap() as u32;

                    let t: Translator = range_translation(source.unwrap(), target.unwrap(), size);
                    translators.push(t);
                }
                "multirange" => {
                    let src_str = config.get(section + ".source").unwrap().as_str().unwrap();
                    let trg_str = config.get(section + ".target").unwrap().as_str().unwrap();
                    let source: Option<char> = getchar(src_str, section);
                    let target: Option<char> = getchar(trg_str, section);
                    let size: u32 = config.get(section + ".size").unwrap().as_int().unwrap() as u32;
                    let slice: u32 = config.get(section + ".slice").unwrap().as_int().unwrap() as u32;
                    let iters: u32 = config.get(section + ".iters").unwrap().as_int().unwrap() as u32;

                    let t: Translator = multirange_translation(source.unwrap(), target.unwrap(),
                                                               size, slice, iters);
                    translators.push(t);
                }
                "lookup" => {
                    let source: &str = config.get(section + ".source").unwrap().as_str().unwrap();
                    let target: &str = config.get(section + ".target").unwrap().as_str().unwrap();

                    if (source.len() != target.len()) {
                        handle_error_ne("Source and target lengths must be equal", section, source.len(), target.len())
                    }

                    let t: Translator = lookup_translation(source, target);
                    translators.push(t);
                }
                value => {
                    handle_error_val("Invalid type", section, value);
                }
            }
        }
    })
}

fn getchar(input: &str, section: &str) -> Option<char> {
    let char_parser = Regex::new(r"\\u\{([0-9a-fA-F]{1,8})}");
    if (input.len() == 1) { input.chars().nth(0) }
    else {
        char_parser.unwrap().find(input).and_then(|pos| {
            Some(char::from_u32(u32::from_str_radix(pos.as_str(), 16).unwrap()).unwrap())
        })
    }
    handle_error_val("Invalid character input (must be one character, or a Unicode codepoint in the format \"\\u{F0000}\")", section, input);
    None
}


fn handle_error(msg: &str, section: &str) {
    eprintln!("[config] Error in section {}: {}", section, msg);
}

fn handle_error_val(msg: &str, section: &str, value: &str) {
    eprintln!("[config] Error in section {}: {} ({})", section, msg, value);
}

fn handle_error_ne(msg: &str, section: &str, left: &str, right: &str) {
    eprintln!("[config] Error in section {}: {} ({} != {})", section, msg, left, right);
}