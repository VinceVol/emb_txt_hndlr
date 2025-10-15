use crate::{BUF_LENGTH, BufError, BufTxt};

impl BufTxt {
    pub fn concat(a: BufTxt, b: BufTxt) -> Result<Self, BufError> {
        let mut new_buf: [u8; BUF_LENGTH] = [' ' as u8; BUF_LENGTH];
        let mut nb_index: usize = 0;
        for i_b in (0..BUF_LENGTH).rev() {
            if b.characters[i_b] == ' ' as u8 {
                nb_index = i_b;
                break;
            }
            new_buf[i_b] = b.characters[i_b];
        }

        for i_a in (0..BUF_LENGTH).rev() {
            if a.characters[i_a] == ' ' as u8 {
                break;
            } else if i_a < (BUF_LENGTH - nb_index) {
                return Err(BufError::BufTooSmall);
            }
            new_buf[i_a - (BUF_LENGTH - nb_index)] = a.characters[i_a];
        }

        return Ok(Self {
            characters: new_buf,
        });
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_concat() {
        let buf_a = BufTxt::new("hello_").unwrap();
        let buf_b = BufTxt::new("world").unwrap();
        let buf_c = BufTxt::concat(buf_a, buf_b).unwrap();
        assert_eq!(
            core::str::from_utf8(&buf_c.characters)
                .unwrap()
                .replace(" ", ""),
            "hello_world"
        );
    }
}
