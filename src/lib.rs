#![cfg_attr(not(test), no_std)]

use core::str;

pub mod numbers;
pub mod strings;

#[derive(PartialEq, Debug, Eq)]
pub enum BufError {
    BufTooSmall,
    UnsignedTooLarge,
    SignedTooLarge,
    FloatTooLarge,
    NumTraitsError,
}

static BUF_LENGTH: usize = 64;
pub struct BufTxt {
    characters: [u8; BUF_LENGTH], //the fixed size bit makes things difficult
}

impl BufTxt {
    pub fn new(s: &str) -> Result<Self, BufError> {
        let buf = s.as_bytes();
        let mut new_buf: [u8; BUF_LENGTH] = [' ' as u8; BUF_LENGTH];

        if buf.len() > BUF_LENGTH {
            return Err(BufError::BufTooSmall);
        }

        for i in (0..BUF_LENGTH).rev() {
            new_buf[i] = buf[buf.len() - (BUF_LENGTH - i)];
            if buf.len() <= BUF_LENGTH - i {
                break;
            }
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
    fn test_new() {
        let new_buf = BufTxt::new("12345.lksjdfhg").expect("Failed to buftxt::new");
        assert_eq!(
            core::str::from_utf8(&new_buf.characters)
                .unwrap()
                .replace(" ", ""),
            "12345.lksjdfhg"
        );
    }
}
