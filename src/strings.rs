use crate::{BUF_LENGTH, BufError, BufTxt};

impl BufTxt {
    pub fn concat(a: BufTxt, b: BufTxt) -> Result<Self, BufError> {
        let mut new_buf: [u8; BUF_LENGTH] = [' ' as u8; BUF_LENGTH];
        let mut nb_index: usize = 0;
        for i_b in (0..BUF_LENGTH).rev() {
            new_buf[i_b] = b.characters[i_b];
            if i_b - 1 > 0 && b.characters[i_b - 1] == ' ' as u8 {
                nb_index = i_b;
                break;
            }
        }

        for i_a in (0..BUF_LENGTH).rev() {
            if i_a < (BUF_LENGTH - nb_index) {
                return Err(BufError::BufTooSmall);
            }

            new_buf[i_a - (BUF_LENGTH - nb_index)] = a.characters[i_a];
            if i_a - 1 > 0 && a.characters[i_a - 1] == ' ' as u8 {
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
}
