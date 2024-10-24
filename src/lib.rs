#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

fn hex_char_to_dec(c: char) -> Option<u8> {
  match c {
    '0'..='9' => Some(c as u8 - '0' as u8),
    'a'..='f' => Some(c as u8 - 'a' as u8 + 10),
    'A'..='F' => Some(c as u8 - 'A' as u8 + 10),
    _ => None,
  }
}

#[napi]
pub fn hex_to_string(hex: String) -> Result<String, napi::Error> {
  if hex.len() % 2 != 0 {
    return Err(napi::Error::from_reason("Invalid hex string"));
  }

  let mut result: String = String::new();
  let chars: Vec<char> = hex.chars().collect();

  for i in (0..chars.len()).step_by(2) {
    let high =
      hex_char_to_dec(chars[i]).ok_or_else(|| napi::Error::from_reason("Invalid hex digit"))?;
    let low =
      hex_char_to_dec(chars[i + 1]).ok_or_else(|| napi::Error::from_reason("Invalid hex digit"))?;

    let byte = (high << 4) | low;
    result.push(byte as char);
  }

  Ok(result)
}

fn dec_to_hex_char(val: u8) -> char {
  match val {
    0..=9 => (val + b'0') as char,
    10..=15 => (val + b'a' - 10) as char,
    _ => panic!("Invalid decimal value"),
  }
}

fn byte_to_hex(byte: u8) -> String {
  let high = (byte >> 4) & 0xF;
  let low = byte & 0xF;
  format!("{}{}", dec_to_hex_char(high), dec_to_hex_char(low))
}

#[napi]
pub fn string_to_hex(s: String) -> String {
  let mut hex: String = String::new();
  for byte in s.bytes() {
    hex.push_str(&byte_to_hex(byte));
  }
  hex
}

#[napi]
pub fn to_uppercase_hex(hex: String) -> String {
  let mut result = String::new();
  for c in hex.chars() {
    result.push(match c {
      'a'..='f' => (c as u8 - b'a' + b'A') as char,
      _ => c,
    });
  }
  result
}

#[napi]
pub fn to_lowercase_hex(hex: String) -> String {
  let mut result = String::new();
  for c in hex.chars() {
    result.push(match c {
      'A'..='F' => (c as u8 - b'A' + b'a') as char,
      _ => c,
    });
  }
  result
}
