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
pub fn decode_hex(hex: String) -> Result<String, napi::Error> {
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
