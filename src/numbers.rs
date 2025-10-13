use libc_print::{libc_dbg, libc_println};
use num_traits::{Float, Num, Signed, ToPrimitive, Unsigned, pow};

#[derive(PartialEq, Debug, Eq)]
#[allow(dead_code)]
enum BufError {
    BufTooSmall,
    UnsignedTooLarge,
    SignedTooLarge,
    FloatTooLarge,
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
        let mut is_neg: bool = false;
        let mut pos_num_i: i64 = num.to_i64().ok_or(BufError::SignedTooLarge)?;
        if num.is_negative() {
            is_neg = true;
            pos_num_i = pos_num_i * -1;
        }

        let pos_num = pos_num_i.to_u64().unwrap(); //tested edge case already
        // libc_println!("The number sent to unsigned was: {}", pos_num);
        match BufTxt::from_u(pos_num) {
            Err(e) => return Err(e),
            Ok(mut buf_txt) => {
                // libc_println!(
                //     "The unsigned fn returned: {}",
                //     core::str::from_utf8(&buf_txt.characters).unwrap()
                // );
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
    fn from_f<T: ToPrimitive + Float>(num: T, d_place: u8) -> Result<Self, BufError> {
        //Pre place decimal point and check for it later when filling in buf
        let mut float_buf: [u8; BUF_LENGTH] = [' ' as u8; BUF_LENGTH];
        float_buf[BUF_LENGTH - (d_place + 1) as usize] = '.' as u8;

        //need to multiply the float to get all the digits we want to cover within a signed num
        //ex 5.4321 -- dec_p of 3 -> 5432.1 -> (5432.1 as signed) = 5432
        let float_num = num.to_f64().ok_or(BufError::FloatTooLarge)?;
        let scaled_num = (pow(10.0, d_place as usize) * float_num)
            .round()
            .to_i64()
            .ok_or(BufError::SignedTooLarge)?;
        let signed_num = BufTxt::from_i(scaled_num)?;
        if scaled_num == 0 {
            return Ok(signed_num);
        }

        //Add 0 to beginning of numbers less than 1
        if float_num < 1.0 && float_num > -1.0 {
            float_buf[BUF_LENGTH - (d_place + 2) as usize] = '0' as u8;
        }

        let mut ii = 0;
        for i in (1..BUF_LENGTH).rev() {
            if signed_num.characters[i] == ' ' as u8 {
                return Ok(Self {
                    characters: float_buf,
                });
            }
            if (float_buf[i] == '.' as u8) || (float_buf[i] == '0' as u8) {
                ii += 1;
            }
            float_buf[i - ii] = signed_num.characters[i];
        }

        return Err(BufError::NumTraitsError);
    }
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
            Ok(res) => std::println!(
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

    #[test]
    fn test_float_buf() {
        let res_0 = BufTxt::from_f(0f32, 2).unwrap();
        assert_eq!(
            core::str::from_utf8(&res_0.characters)
                .unwrap()
                .replace(" ", ""),
            "0"
        );

        let res_over = BufTxt::from_f(std::f64::MAX, 2);
        match res_over {
            Ok(res) => std::println!(
                "The res_over didn't go over and produced: {:?}",
                core::str::from_utf8(&res.characters)
            ),
            Err(e) => assert_eq!(e, BufError::SignedTooLarge),
        }

        let res_random = BufTxt::from_f(215.2341657f32, 3).unwrap();
        assert_eq!(
            core::str::from_utf8(&res_random.characters)
                .unwrap()
                .replace(" ", ""),
            "215.234"
        );

        let neg_res_random = BufTxt::from_f(-3154.52611f32, 2).unwrap();
        assert_eq!(
            core::str::from_utf8(&neg_res_random.characters)
                .unwrap()
                .replace(" ", ""),
            "-3154.53"
        );

        let neg_res_random = BufTxt::from_f(0.52611f32, 2).unwrap();
        assert_eq!(
            core::str::from_utf8(&neg_res_random.characters)
                .unwrap()
                .replace(" ", ""),
            "0.53"
        );
    }
}
