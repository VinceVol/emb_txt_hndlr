use num_traits::{Num, Signed, ToPrimitive, Unsigned};

#[derive(Debug)]
pub enum BufError {
    BufTooSmall,
}

const BUF_LENGTH: usize = 64;
struct BufTxt {
    characters: [u8; BUF_LENGTH], //the fixed size bit makes things difficult
}

impl BufTxt {
    pub fn from_u<T: ToPrimitive + Unsigned>(num: T) -> Result<Self, BufError> {
        let mut int_num: u64 = num.to_u64().unwrap();
        let mut output_buf: [u8; BUF_LENGTH] = [' ' as u8; BUF_LENGTH];
        let mut i = BUF_LENGTH - 1;

        while i > 0 {
            //0x30 is super important otherwise the number shows up as blank lol
            //hexadecimal conversion is 0x30 = 0
            output_buf[i] = (int_num % 10) as u8 + 0x30;
            int_num /= 10;
            i -= 1;
            if int_num == 0 {
                break;
            }
        }
        if int_num > 0 {
            return Err(BufError::BufTooSmall);
        }
        return Ok(BufTxt {
            characters: output_buf,
        });
    }
    // fn from_f64(num: f64) -> Self {}
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_unsigned_buf() {
        let res_0 = BufTxt::from_u(0u8).unwrap();
        assert_eq!(
            core::str::from_utf8(&res_0.characters)
                .unwrap()
                .replace(" ", ""),
            "0"
        );
    }
}
