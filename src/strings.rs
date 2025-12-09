use crate::{BUF_LENGTH, BufError, BufTxt, EMPTY_CELL};

impl BufTxt {
    pub fn concat(a: BufTxt, b: BufTxt) -> Result<Self, BufError> {
        let mut new_buf: [u8; BUF_LENGTH] = [EMPTY_CELL; BUF_LENGTH];
        let mut nb_index: usize = 0;
        for i_b in 0..BUF_LENGTH - 1 {
            new_buf[i_b] = b.characters[i_b];
            println!(
                "b.characters[{}] = {}",
                i_b + 1,
                core::str::from_utf8(&[b.characters[i_b + 1]]).unwrap()
            );
            if b.characters[i_b + 1] == EMPTY_CELL {
                nb_index = i_b;
                break;
            }
        }
        println!("nb_index: {}", nb_index);
        if nb_index == 0 {
            return Err(BufError::BufTooSmall);
        }

        for i_a in 0..BUF_LENGTH - nb_index {
            new_buf[i_a + nb_index] = a.characters[i_a];
            println!("new_buf[{}] = a.characterss[{}]", i_a + nb_index, i_a);
            if a.characters[i_a - 1] == EMPTY_CELL {
                return Ok(Self {
                    characters: new_buf,
                });
            }
        }
        return Err(BufError::BufTooSmall);
    }
    pub fn concat_list(buf_list: &[BufTxt]) -> Result<BufTxt, BufError> {
        let mut new_buf = BufTxt::default();
        for buf in buf_list {
            new_buf = BufTxt::concat(new_buf, *buf)?;
        }
        return Ok(new_buf);
    }
    pub fn split(self, split_c: u8, buf_array: &mut [BufTxt]) -> Result<(), BufError> {
        let mut start_i: usize = 0;
        let mut buf_i: usize = 0;
        for i in 0..BUF_LENGTH {
            if buf_i >= buf_array.len() {
                return Err(BufError::BufTooSmall);
            }
            if i < BUF_LENGTH - 1 && i == start_i + 1 && self.characters[start_i + 1] == split_c {
                //char sol
                buf_array[buf_i] = BufTxt::from_u8(&[self.characters[i - 1]])?;
                buf_i += 1;
                start_i = i + 1;
            } else if self.characters[i] == split_c && (i - start_i > 1) {
                //String Sol
                buf_array[buf_i] = BufTxt::from_u8(&self.characters[start_i..i])?;
                buf_i += 1;
                start_i = i + 1;
            } else if i == BUF_LENGTH - 1 {
                //end sol
                buf_array[buf_i] = BufTxt::from_u8(&self.characters[start_i..BUF_LENGTH])?;
                buf_i += 1;
                start_i = i + 1;
            } else if start_i == i && self.characters[start_i] == split_c {
                //Empty Sol
                start_i = i + 1;
                buf_array[buf_i] = BufTxt::from_str(" ")?;
                buf_i += 1;
            }
        }
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_concat() {
        let buf_a = BufTxt::from_str("0").unwrap();
        let buf_b = BufTxt::from_str(":").unwrap();
        let buf_c = BufTxt::concat(buf_a, buf_b).unwrap();
        assert_eq!(buf_c.to_str().unwrap(), "0:");
        // assert_eq!(
        //     core::str::from_utf8(&buf_c.characters)
        //         .unwrap()
        //         .replace(" ", ""),
        //     "0:"
        // );
    }

    #[test]
    fn test_concat_list() {
        let hr = BufTxt::from_str("0").unwrap();
        let min = BufTxt::from_str("2").unwrap();
        let sec = BufTxt::from_str("20").unwrap();
        let colon = BufTxt::from_str(":").unwrap();
        let combined = BufTxt::concat_list(&[hr, colon, min, colon, sec]).unwrap();
        assert_eq!(combined.to_str().unwrap(), "0:2:20");
        // assert_eq!(
        //     core::str::from_utf8(&combined.characters)
        //         .unwrap()
        //         .replace(" ", ""),
        //     "0:2:20"
        // );
    }

    #[test]
    #[ignore]
    fn test_split() {
        let gpg_txt = BufTxt::from_str(
            "GPGGA,,113727,4303.16727,N,08612.65632,W,1,07,1.43,197.6,M,-34.5,M,,*60",
        )
        .unwrap();
        let mut buf_list: [BufTxt; 30] = [BufTxt::default(); 30];
        println!("GPGGA,,113727,4303.16727,N,08612.65632,W,1,07,1.43,197.6,M,-34.5,M,,*60");
        let _ = gpg_txt.split(',' as u8, &mut buf_list);
        for item in buf_list {
            println!(
                "{}",
                core::str::from_utf8(&item.characters)
                    .unwrap()
                    .replace(" ", "")
            );
        }
        assert_eq!(true, false);
    }
}

//Print Debug
// println!(
//     "start_i|{}: {}",
//     start_i,
//     core::str::from_utf8(&[self.characters[start_i]]).unwrap()
// );
// println!(
//     "      i|{}: {}",
//     i,
//     core::str::from_utf8(&[self.characters[i]]).unwrap()
// );
// //----------------------------------------------------------------------------------
// println!("string sol");
// for sp in 1..start_i {
//     print!(" ");
// }
// print!("s");
// for sp in start_i..i {
//     print!(" ");
// }
// println!("i");
// println!("{}", core::str::from_utf8(&self.characters).unwrap());
// //----------------------------------------------------------------------------------
