#![deny(clippy::all)]

use std::collections::HashMap;
#[macro_use]
extern crate napi_derive;
#[macro_use]
extern crate lazy_static;

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

#[napi]
pub fn decode_hex(hex: String) -> Result<String, napi::Error> {
  if hex.len() % 2 != 0 {
    return Err(napi::Error::from_reason("Invalid hex string"));
  }

  let mut result: String = String::new();
  let chars: Vec<char> = hex.chars().collect();

  for i in (0..chars.len()).step_by(2) {
    let high = HEX_TO_DEC
      .get(&chars[i])
      .ok_or_else(|| napi::Error::from_reason("Invalid hex digit"))?;
    let low = HEX_TO_DEC
      .get(&chars[i + 1])
      .ok_or_else(|| napi::Error::from_reason("Invalid hex digit"))?;

    let byte: u8 = (high << 4) | low;
    result.push(byte as char);
  }

  Ok(result)
}

#[napi]
pub fn encode_hex_upper_case(hex: String) -> String {
  let mut result: String = String::new();
  for c in hex.chars() {
    result.push(match c {
      'a'..='f' => (c as u8 - b'a' + b'A') as char,
      _ => c,
    });
  }
  result
}

#[napi]
pub fn encode_hex_lower_case(hex: String) -> String {
  let mut result: String = String::new();
  for c in hex.chars() {
    result.push(match c {
      'A'..='F' => (c as u8 - b'A' + b'a') as char,
      _ => c,
    });
  }
  result
}
