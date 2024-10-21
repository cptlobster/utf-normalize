/// Homograph translator function generators.

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
/// translators, as it will use a modulus to collapse the adjacent ranges into one range. It can
/// also handle non-adjacent ranges (i.e. multiple uppercase ranges separated by lowercase ranges)
/// using the `slice` parameter.
///
/// ## Example
/// We can create a multi-range translator to handle some of the characters in the Mathematical
/// Alphanumeric Symbols block.
/// ```rs
/// todo!("Write example code");
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

/// Run a chain of translators on a single character.
pub fn translate(source: char, translator: &[Translator]) -> char {
    let ord: u32 = source as u32;
    char::from_u32(translator.iter().flat_map(|f| (*f)(ord)).next().unwrap_or(ord)).unwrap_or(source)
}