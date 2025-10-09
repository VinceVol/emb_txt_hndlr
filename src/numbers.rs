use num_traits::{Float, Num, Signed, ToPrimitive, Unsigned};

#[derive(PartialEq, Debug, Eq)]
#[allow(dead_code)]
enum BufError {
    BufTooSmall,
    UnsignedTooLarge,
    SignedTooLarge,
    NumTraitsError,
}

const BUF_LENGTH: usize = 64;
#[allow(dead_code)]
pub struct BufTxt {
    characters: [u8; BUF_LENGTH], //the fixed size bit makes things difficult
}

#[allow(dead_code)]
impl BufTxt {
    pub fn from_u<T: ToPrimitive + Unsigned>(num: T) -> Result<Self, BufError> {
        let mut int_num: u64;
        if let Some(int) = num.to_u64() {
            int_num = int;
            // println!("input:  \nConversion to u64: {}", int);
        } else {
            return Err(BufError::UnsignedTooLarge);
        }

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
    fn from_i<T: ToPrimitive + Signed>(num: T) -> Result<Self, BufError> {
        if num.to_i128().unwrap() > i64::MAX as i128 {
            return Err(BufError::SignedTooLarge);
        }
        let mut is_neg: bool = false;
        let mut pos_num_i: i64 = num.to_i64().unwrap();
        if num.is_negative() {
            is_neg = true;
            pos_num_i = pos_num_i * -1;
        }

        let pos_num = pos_num_i.to_u64().unwrap(); //tested edge case already
        println!("The number sent to unsigned was: {}", pos_num);
        match BufTxt::from_u(pos_num) {
            Err(e) => return Err(e),
            Ok(mut buf_txt) => {
                println!(
                    "The unsigned fn returned: {}",
                    core::str::from_utf8(&buf_txt.characters)
                        .unwrap()
                        .replace(" ", "")
                );
                if !is_neg {
                    return Ok(buf_txt);
                }
                for i in (0..BUF_LENGTH).rev() {
                    if buf_txt.characters[i] == (' ' as u8) {
                        buf_txt.characters[i] = '-' as u8;
                        return Ok(buf_txt);
                    }
                }
            }
        }

        return Err(BufError::BufTooSmall);
    }
    // fn from_f<T: ToPrimitive + Float>(num: T) -> Result<Self, BufError> {}
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

        let res_over = BufTxt::from_u(std::u128::MAX);
        assert_eq!(res_over.err().unwrap(), BufError::UnsignedTooLarge);

        let res_random = BufTxt::from_u(215u8).unwrap();
        assert_eq!(
            core::str::from_utf8(&res_random.characters)
                .unwrap()
                .replace(" ", ""),
            "215"
        );
    }

    #[test]
    fn test_signed_buf() {
        let res_0 = BufTxt::from_i(0i8).unwrap();
        assert_eq!(
            core::str::from_utf8(&res_0.characters)
                .unwrap()
                .replace(" ", ""),
            "0"
        );

        let res_over = BufTxt::from_i(std::i128::MAX);
        match res_over {
            Ok(res) => println!(
                "The res_over didn't go over and produced: {:?}",
                core::str::from_utf8(&res.characters)
            ),
            Err(e) => assert_eq!(e, BufError::SignedTooLarge),
        }

        let res_random = BufTxt::from_i(215i16).unwrap();
        assert_eq!(
            core::str::from_utf8(&res_random.characters)
                .unwrap()
                .replace(" ", ""),
            "215"
        );

        let neg_res_random = BufTxt::from_i(-3154i32).unwrap();
        assert_eq!(
            core::str::from_utf8(&neg_res_random.characters)
                .unwrap()
                .replace(" ", ""),
            "-3154"
        );
    }
}
