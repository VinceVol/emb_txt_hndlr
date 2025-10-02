#![no_main]

use num_traits::{Num, Signed, ToPrimitive, Unsigned};

pub enum BufError {
    BufTooSmall,
}

const BUF_LENGTH: usize = 64;
struct BufTxt {
    characters: [u8; BUF_LENGTH], //the fixed size bit makes things difficult
}

impl BufTxt {
    fn from_u<T: ToPrimitive + Unsigned>(num: T) -> Result<Self, BufError> {
        let mut int_num: u64 = num.to_u64().unwrap();
        let mut output_buf: [u8; BUF_LENGTH] = [0; BUF_LENGTH];
        let mut i = BUF_LENGTH - 1;

        while i > 0 {
            //0x30 is super important otherwise the number shows up as blank lol
            //hexadecimal conversion is 0x30 = 0
            output_buf[i] = (int_num % 10) as u8 + 0x30;
            int_num /= 10;
            i -= 1;
        }
        return Ok(BufTxt {
            characters: output_buf,
        });
    }
    fn from_f64(num: f64) -> Self {}
}
