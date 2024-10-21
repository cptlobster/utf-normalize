pub type Translator = Box<dyn Fn(u32) -> Option<u32>>;

pub fn range_translation(source: char, target: char, size: u32) -> Translator {
    let s: u32 = source as u32;
    let t: u32 = target as u32;
    Box::new(
        move |ord: u32| {
            let is_in_rt: bool = ord >= s && ord <= s + size - 1;
            if (is_in_rt) { Some(ord - s + t) } else { None }
        }
    )
}

pub fn multirange_translation(source: char, target: char, size: u32, slice: u32, iters: u32) -> Translator {
    let s: u32 = source as u32;
    let t: u32 = target as u32;
    Box::new(
        move |ord: u32| {
            let is_in_mrt: bool = ord >= s && ord <= s + (slice * iters) - 1;
            if (is_in_mrt) {
                let ord_ir = (ord - s) % slice;
                let is_in_rt: bool = ord_ir <= s + size - 1;
                if (is_in_rt) { Some(ord_ir + t) } else { None }
            }
            else { None }
        }
    )
}

pub fn table_translation(source: &str, target: &str) -> Translator {
    Box::new(
        move |ord: u32| {
            None
        }
    )
}

pub fn translate(source: char, translator: &[Translator]) -> char {
    let ord: u32 = source as u32;
    char::from_u32(translator.iter().flat_map(|f| (*f)(ord)).next().unwrap_or(ord)).unwrap_or(source)
}