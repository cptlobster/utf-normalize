/// Function generators for Unicode homoglyph normalization.
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

/// The Translator is an alias for a function that converts a UTF-32 codepoint (represented as a
/// `u32`) to another UTF-32 codepoint. How this conversion is achieved is an exercise left to the
/// developer (although some translator generator functions are provided in this module).
///
/// To make this as configurable as possible, a translator does not (and should not!) handle the
/// entire UTF-32 character set. If a character passed into a translator matches a codepoint that
/// the translator is designed to handle, it will return a `Some(u32)` containing the translated
/// value. Otherwise, it will return a `None`. Therefore, translators can be chained using an
/// ordered data structure (such as a `Vec` or array) and iterators. The `translate()` function
/// uses `flat_map` on an iterator of translators to lazily evaluate and return on the first
/// successful translation.
pub type Translator = Box<dyn Fn(u32) -> Option<u32>>;

/// This is a naive lookup table translator. It takes two strings of characters, and if the input
/// matches one of the characters in the table, it returns the output character at the same index.
pub fn lookup_translation(source: &str, target: &str) -> Translator {
    todo!("Implement lookup table translator");
    Box::new(
        move |ord: u32| {
            None
        }
    )
}

/// Although the lookup table works fine for arbitrary groups of characters, it still has to go
/// through an entire string to find a match. The range translator optimizes the table approach by
/// assuming that all the characters in the table are sequential. Therefore, translating a character
/// is as simple as subtracting the offset between the two ranges.
///
/// ## Example
/// We can create a range translator that converts all lowercase characters to uppercase:
/// ```rs
/// let tr_to_uppercase: Translator = range_translation('a', 'A', 26);
/// ```
pub fn range_translation(source: char, target: char, size: u32) -> Translator {
    let s: u32 = source as u32;
    let t: u32 = target as u32;
    let offset: u32 = s - t;
    Box::new(
        move |ord: u32| {
            let is_in_rt: bool = ord >= s && ord <= s + size - 1;
            if (is_in_rt) { Some(ord - offset) } else { None }
        }
    )
}

/// The multi-range translator is primarily useful for cases such as the Mathematical Alphanumeric
/// Symbols block, where there are several different formats of what are essentially the same
/// letters right next to each other. This is more efficient than chaining multiple range
/// translators, as it will use a modulus to collapse the adjacent ranges into one range rather than
/// checking every range independently.
///
/// It can also handle non-adjacent ranges (i.e. multiple uppercase ranges separated by lowercase
/// ranges) by providing different values for `slice` and `size`, where a larger `slice` value will
/// skip `slice - size - 1` characters after each range.
///
/// ## Example
/// We can create a multi-range translator to handle some of the characters in the Mathematical
/// Alphanumeric Symbols block.
/// ```rs
/// // Mathematical bold, italic, bold/italic; uppercase only. this will skip over the lowercase
/// // letters because of the `slice` parameter
/// let tr_upper: Translator = multirange_translation('\u{1D400}', 'A', 26, 52, 3),
/// // Mathematical bold, italic, bold/italic; lowercase only. this will skip over the uppercase
/// // letters because of the `slice` parameter
/// let tr_lower: Translator = multirange_translation('\u{1D41A}', 'a', 26, 52, 3),
/// ```
pub fn multirange_translation(source: char, target: char, size: u32, slice: u32, iters: u32) -> Translator {
    let s: u32 = source as u32;
    let t: u32 = target as u32;
    Box::new(
        move |ord: u32| {
            let is_in_mrt: bool = ord >= s && ord <= s + (slice * iters) - 1;
            if (is_in_mrt) {
                let ord_ir: u32 = (ord - s) % slice;
                let is_in_rt: bool = ord_ir <= s + size - 1;
                if (is_in_rt) { Some(ord_ir + t) } else { None }
            }
            else { None }
        }
    )
}

/// The ASCII filter should be placed at the front of a translator list. If you do not intend to
/// match against any ASCII characters, this filter will return if a character is ASCII. This is an
/// optimization, as otherwise it would have to run through all of the translators before returning.
pub fn ascii_filter() -> Translator {
    let ascii_ub: u32 = 128; // should I adjust this to allow for ASCII extended chars?
    Box::new(
        move |ord: u32| {
            if (ord < ascii_ub) { Some(ord) } else { None }
        }
    )
}

/// Run a chain of translators on a single character.
pub fn translate(source: char, translator: &[Translator]) -> char {
    let ord: u32 = source as u32;
    char::from_u32(translator.iter().flat_map(|f| (*f)(ord)).next().unwrap_or(ord))
        .unwrap_or(source)
}

/// Run a chain of translators on a single character.
pub fn translate_vec(source: char, translator: &Vec<Translator>) -> char {
    let ord: u32 = source as u32;
    char::from_u32(translator.iter().flat_map(|f| (*f)(ord)).next().unwrap_or(ord))
        .unwrap_or(source)
}

/// Run a single translator on a single character. If you want to use multiple translators, you
/// should use `translate()` with an array of translators.
pub fn translate_one(source: char, translator: &Translator) -> char {
    let ord: u32 = source as u32;
    match translator(ord) {
        Some(res0) => { char::from_u32(res0).unwrap_or(source) }
        None => { source }
    }
}