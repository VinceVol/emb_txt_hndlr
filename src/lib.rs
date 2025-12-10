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

pub static BUF_LENGTH: usize = 256;
static EMPTY_CELL: u8 = 0;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BufTxt {
    pub characters: [u8; BUF_LENGTH], //the fixed size bit makes things difficult
}

impl BufTxt {
    pub fn to_str(&self) -> Option<&str> {
        core::ffi::CStr::from_bytes_until_nul(&self.characters)
            .ok()?
            .to_str()
            .ok()
    }
    pub fn from_str(s: &str) -> Result<Self, BufError> {
        let buf = s.as_bytes();
        return BufTxt::from_u8(buf);
    }
    pub fn from_u8(buf: &[u8]) -> Result<Self, BufError> {
        let mut new_buf: [u8; BUF_LENGTH] = [EMPTY_CELL; BUF_LENGTH];

        if buf.len() > BUF_LENGTH {
            return Err(BufError::BufTooSmall);
        }

        // println!("buf.len():{} \n BUF_LENGTH:{}", buf.len(), BUF_LENGTH);
        for i in 0..BUF_LENGTH {
            if i >= buf.len() {
                break;
            }
            new_buf[i] = buf[i];
        }

        return Ok(Self {
            characters: new_buf,
        });
    }
}

impl Default for BufTxt {
    fn default() -> Self {
        Self {
            characters: [EMPTY_CELL; BUF_LENGTH],
        }
    }
}

#[cfg(test)]
mod tests {
    use core::ffi;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_new() {
        let new_buf = BufTxt::from_str("12345.lksjdfhg").expect("Failed to buftxt::new");
        assert_eq!(new_buf.to_str().unwrap(), "12345.lksjdfhg");
    }
}
