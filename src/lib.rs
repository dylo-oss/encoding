#![deny(clippy::all)]

pub mod base32;
pub mod base64;
pub mod hex;

use std::collections::HashMap;
#[macro_use]
extern crate napi_derive;
#[macro_use]
extern crate lazy_static;

const BASE32_UPPER: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
const BASE32_LOWER: &[u8; 32] = b"abcdefghijklmnopqrstuvwxyz234567";
pub const BASE64: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
pub const BASE64_URL: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

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

pub const DECODE_MAP_64: [i8; 256] = {
  let mut map: [i8; 256] = [-1i8; 256];
  map[b'A' as usize] = 0;
  map[b'B' as usize] = 1;
  map[b'C' as usize] = 2;
  map[b'D' as usize] = 3;
  map[b'E' as usize] = 4;
  map[b'F' as usize] = 5;
  map[b'G' as usize] = 6;
  map[b'H' as usize] = 7;
  map[b'I' as usize] = 8;
  map[b'J' as usize] = 9;
  map[b'K' as usize] = 10;
  map[b'L' as usize] = 11;
  map[b'M' as usize] = 12;
  map[b'N' as usize] = 13;
  map[b'O' as usize] = 14;
  map[b'P' as usize] = 15;
  map[b'Q' as usize] = 16;
  map[b'R' as usize] = 17;
  map[b'S' as usize] = 18;
  map[b'T' as usize] = 19;
  map[b'U' as usize] = 20;
  map[b'V' as usize] = 21;
  map[b'W' as usize] = 22;
  map[b'X' as usize] = 23;
  map[b'Y' as usize] = 24;
  map[b'Z' as usize] = 25;
  map[b'a' as usize] = 26;
  map[b'b' as usize] = 27;
  map[b'c' as usize] = 28;
  map[b'd' as usize] = 29;
  map[b'e' as usize] = 30;
  map[b'f' as usize] = 31;
  map[b'g' as usize] = 32;
  map[b'h' as usize] = 33;
  map[b'i' as usize] = 34;
  map[b'j' as usize] = 35;
  map[b'k' as usize] = 36;
  map[b'l' as usize] = 37;
  map[b'm' as usize] = 38;
  map[b'n' as usize] = 39;
  map[b'o' as usize] = 40;
  map[b'p' as usize] = 41;
  map[b'q' as usize] = 42;
  map[b'r' as usize] = 43;
  map[b's' as usize] = 44;
  map[b't' as usize] = 45;
  map[b'u' as usize] = 46;
  map[b'v' as usize] = 47;
  map[b'w' as usize] = 48;
  map[b'x' as usize] = 49;
  map[b'y' as usize] = 50;
  map[b'z' as usize] = 51;
  map[b'0' as usize] = 52;
  map[b'1' as usize] = 53;
  map[b'2' as usize] = 54;
  map[b'3' as usize] = 55;
  map[b'4' as usize] = 56;
  map[b'5' as usize] = 57;
  map[b'6' as usize] = 58;
  map[b'7' as usize] = 59;
  map[b'8' as usize] = 60;
  map[b'9' as usize] = 61;
  map[b'+' as usize] = 62;
  map[b'/' as usize] = 63;
  map[b'-' as usize] = 62;
  map[b'_' as usize] = 63;
  map
};
