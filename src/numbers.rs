use num_traits::{Float, Num, Signed, Signed, ToPrimitive, Unsigned};

#[derive(PartialEq, Debug, Eq)]
#[allow(dead_code)]
enum BufError {
    BufTooSmall,
    UnsignedTooLarge,
    num_traits_error,
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
            println!("input:  \nConversion to u64: {}", int);
        } else {
            return Err(BufError::UnsignedTooLarge);
        }

        let mut output_buf: [u8; BUF_LENGTH] = [' ' as u8; BUF_LENGTH];
        let mut i = BUF_LENGTH - 1;

        while i > 0 {
            println!("int: {}\nint%10: {}", int_num, int_num % 10);
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
        let u_buf: [u8; BUF_LENGTH];
        let mut is_neg: bool = false;
        let mut pos_num: u64 = 0;
        if num.is_negative() {
            is_neg = true;
            if let Some(u_num) = num.to_u64() {
                pos_num = u_num;
            } else {
                return Err(BufError::num_traits_error);
            }
        }
        if let Ok(mut buf_txt) = BufTxt::from_u(pos_num) {
            buf_txt.characters.reverse();
            todo!() //.find doesn't return an index
            let index = BUF_LENGTH
                - *buf_txt
                    .characters
                    .iter()
                    .find(|&&x| x == ' ' as u8)
                    .unwrap() as usize;
        }

        return Err(BufError::UnsignedTooLarge);
    }
    fn from_f<T: ToPrimitive + Float>(num: T) -> Result<Self, BufError> {}
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

        let res_max = BufTxt::from_u(std::u128::MAX);
        assert_eq!(res_max.err().unwrap(), BufError::UnsignedTooLarge);

        println!("Input: 215");
        let res_random = BufTxt::from_u(215u8).unwrap();
        assert_eq!(
            core::str::from_utf8(&res_random.characters)
                .unwrap()
                .replace(" ", ""),
            "215"
        );
    }
}
