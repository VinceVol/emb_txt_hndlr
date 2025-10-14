#![cfg_attr(not(test), no_std)]

pub mod numbers;
pub mod strings;

static BUF_LENGTH: usize = 64;
pub struct BufTxt {
    characters: [u8; BUF_LENGTH], //the fixed size bit makes things difficult
}
