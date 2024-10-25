#![deny(clippy::all)]

pub mod base32;
pub mod hex;

use std::collections::HashMap;
#[macro_use]
extern crate napi_derive;
#[macro_use]
extern crate lazy_static;

const BASE32_UPPER: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
const BASE32_LOWER: &[u8; 32] = b"abcdefghijklmnopqrstuvwxyz234567";

const PADDING: u8 = b'=';

const DECODE_MAP: [i8; 256] = {
  let mut map: [i8; 256] = [-1i8; 256];
  let mut i: usize = 0;
  while i < 32 {
    map[BASE32_UPPER[i] as usize] = i as i8;
    map[BASE32_LOWER[i] as usize] = i as i8;
    i += 1;
  }
  map
};

lazy_static! {
  static ref HEX_TO_DEC: HashMap<char, u8> = {
    let mut m = HashMap::new();

    for i in 0..=9 {
      m.insert(char::from_digit(i, 10).unwrap(), i as u8);
    }

    for i in 0..=5 {
      m.insert((b'a' + i) as char, 10 + i as u8);
    }

    for i in 0..=5 {
      m.insert((b'A' + i) as char, 10 + i as u8);
    }

    m
  };
}
