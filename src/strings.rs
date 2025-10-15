use crate::{BUF_LENGTH, BufError, BufTxt};

impl BufTxt {
    pub fn concat(a: BufTxt, b: BufTxt) -> Result<Self, BufError> {
        let mut new_buf: [u8; BUF_LENGTH] = [0; BUF_LENGTH];
        let mut nb_index: usize = 0;
        for i_a in (0..BUF_LENGTH).rev() {
            if a.characters[i_a] == 0 {
                nb_index = i_a;
                break;
            }
            new_buf[i_a] = a.characters[i_a];
        }

        for i_b in (0..BUF_LENGTH).rev() {
            if b.characters[i_b] == 0 {
                break;
            } else if i_b < (BUF_LENGTH - nb_index) {
                return Err(BufError::BufTooSmall);
            }
            new_buf[i_b - (BUF_LENGTH - nb_index)] = b.characters[i_b];
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

    // #[test]
    // fn test_concat() {
    //     let buf_a = BUF
    // }
}
