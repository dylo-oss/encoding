use crate::{BASE32_LOWER, BASE32_UPPER, DECODE_MAP, PADDING};

fn encode_base32(input: &[u8], alphabet: &[u8], padding: bool) -> String {
  let mut result: String = String::new();
  let mut i: usize = 0;

  while i < input.len() {
    let mut buffer: u64 = 0;
    let mut buffer_bit_size: i32 = 0;

    for j in 0..5 {
      if i + j < input.len() {
        buffer = (buffer << 8) | input[i + j] as u64;
        buffer_bit_size += 8;
      }
    }

    if buffer_bit_size % 5 != 0 {
      buffer <<= 5 - (buffer_bit_size % 5);
      buffer_bit_size += 5 - (buffer_bit_size % 5);
    }

    for _ in 0..8 {
      if buffer_bit_size >= 5 {
        let index: usize = ((buffer >> (buffer_bit_size - 5)) & 0x1F) as usize;
        result.push(alphabet[index] as char);
        buffer_bit_size -= 5;
      } else if buffer_bit_size > 0 {
        let index: usize = ((buffer << (5 - buffer_bit_size)) & 0x1F) as usize;
        result.push(alphabet[index] as char);
        buffer_bit_size = 0;
      } else if padding {
        result.push(PADDING as char);
      }
    }

    i += 5;
  }

  result
}

#[napi]
pub fn encode_base32_uppercase(input: &[u8]) -> String {
  encode_base32(input, BASE32_UPPER, true)
}

#[napi]
pub fn encode_base32_lowercase(input: &[u8]) -> String {
  encode_base32(input, BASE32_LOWER, true)
}

#[napi]
pub fn encode_base32_lowercase_no_padding(input: &[u8]) -> String {
  encode_base32(input, BASE32_LOWER, false)
}

#[napi]
pub fn encode_base32_uppercase_no_padding(input: &[u8]) -> String {
  encode_base32(input, BASE32_UPPER, false)
}

fn decode_base32_internal(input: &[u8], ignore_padding: bool) -> Result<Vec<u8>, &'static str> {
  let mut buffer: u32 = 0u32;
  let mut bits_left: i32 = 0;
  let mut output: Vec<u8> = Vec::new();

  for &byte in input {
    if byte == PADDING {
      if !ignore_padding {
        break;
      } else {
        continue;
      }
    }

    let value: i8 = DECODE_MAP[byte as usize];
    if value == -1 {
      return Err("Invalid Base32 character");
    }

    buffer = (buffer << 5) | value as u32;
    bits_left += 5;

    if bits_left >= 8 {
      let byte: u8 = (buffer >> (bits_left - 8)) as u8;
      output.push(byte);
      bits_left -= 8;
    }
  }

  Ok(output)
}

#[napi]
pub fn decode_base32(input: String) -> Result<Vec<u8>, napi::Error> {
  let bytes: &[u8] = input.as_bytes();
  decode_base32_internal(bytes, false).map_err(|e: &str| napi::Error::from_reason(e.to_string()))
}

#[napi]
pub fn decode_base32_ignore_padding(input: String) -> Result<Vec<u8>, napi::Error> {
  let bytes: &[u8] = input.as_bytes();
  decode_base32_internal(bytes, true).map_err(|e: &str| napi::Error::from_reason(e.to_string()))
}
