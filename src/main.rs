#![no_main]

mod float;

pub enum BufError {
    BufTooSmall,
}

struct BufTxt {
    characters: [u8; 64], //the fixed size bit makes things difficult
}

impl BufTxt {
    fn from_u<T: Copy + Sized>(num: T) -> Self {
        return BufTxt {
            characters: [0; 64],
        };
    }
    fn from_f64(num: f64) -> Self {}
}
