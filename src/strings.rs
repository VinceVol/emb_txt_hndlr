use crate::{BUF_LENGTH, BufError, BufTxt, EMPTY_CELL};

impl BufTxt {
    pub fn concat(a: BufTxt, b: BufTxt) -> Result<Self, BufError> {
        let mut new_buf: [u8; BUF_LENGTH] = [EMPTY_CELL; BUF_LENGTH];
        let mut nb_index: usize = 0;
        for i_b in (0..BUF_LENGTH).rev() {
            new_buf[i_b] = b.characters[i_b];
            if i_b - 1 > 0 && b.characters[i_b - 1] == EMPTY_CELL {
                nb_index = i_b;
                break;
            }
        }

        for i_a in (0..BUF_LENGTH).rev() {
            if i_a < (BUF_LENGTH - nb_index) {
                return Err(BufError::BufTooSmall);
            }

            new_buf[i_a - (BUF_LENGTH - nb_index)] = a.characters[i_a];
            if i_a - 1 > 0 && a.characters[i_a - 1] == EMPTY_CELL {
                break;
            }
        }

        return Ok(Self {
            characters: new_buf,
        });
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
                buf_array[buf_i] =
                    BufTxt::new(core::str::from_utf8(&[self.characters[i - 1]]).unwrap())?;
                buf_i += 1;
                start_i = i + 1;
            } else if (self.characters[i] == split_c && (i - start_i > 1)) {
                //String Sol
                buf_array[buf_i] =
                    BufTxt::new(core::str::from_utf8(&self.characters[start_i..i]).unwrap())?;
                buf_i += 1;
                start_i = i + 1;
            } else if i == BUF_LENGTH - 1 {
                //end sol
                buf_array[buf_i] = BufTxt::new(
                    core::str::from_utf8(&self.characters[start_i..BUF_LENGTH]).unwrap(),
                )?;
                buf_i += 1;
                start_i = i + 1;
            } else if start_i == i && self.characters[start_i] == split_c {
                //Empty Sol
                start_i = i + 1;
                buf_array[buf_i] = BufTxt::new(" ")?;
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
        let buf_a = BufTxt::new("0").unwrap();
        let buf_b = BufTxt::new(":").unwrap();
        let buf_c = BufTxt::concat(buf_a, buf_b).unwrap();
        assert_eq!(
            core::str::from_utf8(&buf_c.characters)
                .unwrap()
                .replace(" ", ""),
            "0:"
        );
    }

    #[test]
    fn test_concat_list() {
        let hr = BufTxt::new("0").unwrap();
        let min = BufTxt::new("2").unwrap();
        let sec = BufTxt::new("20").unwrap();
        let colon = BufTxt::new(":").unwrap();
        let combined = BufTxt::concat_list(&[hr, colon, min, colon, sec]).unwrap();
        assert_eq!(
            core::str::from_utf8(&combined.characters)
                .unwrap()
                .replace(" ", ""),
            "0:2:20"
        );
    }

    #[test]
    fn test_split() {
        let gpg_txt =
            BufTxt::new("GPGGA,,113727,4303.16727,N,08612.65632,W,1,07,1.43,197.6,M,-34.5,M,,*60")
                .unwrap();
        let mut buf_list: [BufTxt; 30] = [BufTxt::default(); 30];
        println!("GPGGA,,113727,4303.16727,N,08612.65632,W,1,07,1.43,197.6,M,-34.5,M,,*60");
        gpg_txt.split(',' as u8, &mut buf_list);
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
