use crate::{BASE64, BASE64_URL, DECODE_MAP_64, PADDING};

fn encode_base64(input: &[u8], alphabet: &[u8], padding: bool) -> String {
  let mut result: String = String::new();
  let mut i: usize = 0;

  while i < input.len() {
    let mut buffer: u32 = 0;
    let mut buffer_bit_size: i32 = 0;

    for j in 0..3 {
      if i + j < input.len() {
        buffer = (buffer << 8) | input[i + j] as u32;
        buffer_bit_size += 8;
      }
    }

    if buffer_bit_size % 6 != 0 {
      buffer <<= 6 - (buffer_bit_size % 6);
      buffer_bit_size += 6 - (buffer_bit_size % 6);
    }

    for _ in 0..4 {
      if buffer_bit_size >= 6 {
        let index: usize = ((buffer >> (buffer_bit_size - 6)) & 0x3F) as usize;
        result.push(alphabet[index] as char);
        buffer_bit_size -= 6;
      } else if buffer_bit_size > 0 {
        let index: usize = ((buffer << (6 - buffer_bit_size)) & 0x3F) as usize;
        result.push(alphabet[index] as char);
        buffer_bit_size = 0;
      } else if padding {
        result.push(PADDING as char);
      }
    }

    i += 3;
  }

  result
}

fn decode_base64_internal(input: &[u8], ignore_padding: bool) -> Result<Vec<u8>, &'static str> {
  let mut buffer: u32 = 0;
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

    let value: i8 = DECODE_MAP_64[byte as usize];
    if value == -1 {
      return Err("Invalid Base64 character");
    }

    buffer = (buffer << 6) | value as u32;
    bits_left += 6;

    if bits_left >= 8 {
      output.push((buffer >> (bits_left - 8)) as u8);
      bits_left -= 8;
    }
  }

  Ok(output)
}

#[napi]
pub fn encode_base64_standard(input: &[u8]) -> String {
  encode_base64(input, BASE64, true)
}

#[napi]
pub fn encode_base64_url(input: &[u8]) -> String {
  encode_base64(input, BASE64_URL, true)
}

#[napi]
pub fn encode_base64_standard_no_padding(input: &[u8]) -> String {
  encode_base64(input, BASE64, false)
}

#[napi]
pub fn encode_base64_url_no_padding(input: &[u8]) -> String {
  encode_base64(input, BASE64_URL, false)
}

#[napi]
pub fn decode_base64(input: String) -> Result<Vec<u8>, napi::Error> {
  let bytes: &[u8] = input.as_bytes();
  decode_base64_internal(bytes, false).map_err(|e: &str| napi::Error::from_reason(e.to_string()))
}

#[napi]
pub fn decode_base64_ignore_padding(input: String) -> Result<Vec<u8>, napi::Error> {
  let bytes: &[u8] = input.as_bytes();
  decode_base64_internal(bytes, true).map_err(|e: &str| napi::Error::from_reason(e.to_string()))
}
