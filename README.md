# Base Encoding/Decoding Library

A high-performance Node.js native addon for base encoding and decoding operations, built with NAPI-RS. This library provides fast, secure, and memory-efficient implementations for Base32, Base64, and Hex encoding/decoding operations.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- 🚀 High-performance native implementations
- 💪 Written in Rust, exposed to Node.js via NAPI-RS
- 🔒 Memory-safe operations
- 📦 Zero dependencies
- 🎯 TypeScript type definitions included
- 🔄 Support for various encoding formats:
  - Base32 (uppercase/lowercase, with/without padding)
  - Base64 (standard/URL-safe, with/without padding)
  - Hex (uppercase/lowercase)

## Installation

```bash
npm install @dylo/encoding
# or
yarn add @dylo/encoding
# or
pnpm add @dylo/encoding
```

## API Reference

### Base32 Functions

#### Encoding

```typescript
function encodeBase32Uppercase(input: Uint8Array): string;
function encodeBase32Lowercase(input: Uint8Array): string;
function encodeBase32LowercaseNoPadding(input: Uint8Array): string;
function encodeBase32UppercaseNoPadding(input: Uint8Array): string;
```

These functions encode a `Uint8Array` to a Base32 string with different formats:

- Uppercase with padding (standard)
- Lowercase with padding
- Lowercase without padding
- Uppercase without padding

#### Decoding

```typescript
function decodeBase32(input: string): Array<number>;
function decodeBase32IgnorePadding(input: string): Array<number>;
```

Convert Base32 strings back to byte arrays, with options to handle padded and unpadded input.

### Base64 Functions

#### Encoding

```typescript
function encodeBase64Standard(input: Uint8Array): string;
function encodeBase64Url(input: Uint8Array): string;
function encodeBase64StandardNoPadding(input: Uint8Array): string;
function encodeBase64UrlNoPadding(input: Uint8Array): string;
```

Encode binary data to Base64 strings with options for:

- Standard Base64 encoding (with padding)
- URL-safe Base64 encoding (with padding)
- Standard Base64 encoding (without padding)
- URL-safe Base64 encoding (without padding)

#### Decoding

```typescript
function decodeBase64(input: string): Array<number>;
function decodeBase64IgnorePadding(input: string): Array<number>;
```

Convert Base64 strings back to byte arrays, with options to handle padded and unpadded input.

### Hex Functions

```typescript
function decodeHex(hex: string): string;
function encodeHexUpperCase(hex: string): string;
function encodeHexLowerCase(hex: string): string;
```

Convert between hexadecimal strings and binary data, with options for uppercase or lowercase output.

## Usage Examples

```typescript
import { encodeBase32Uppercase, decodeBase32, encodeBase64Standard, decodeBase64, encodeHexUpperCase } from '@dylo/encoding';

// Base32 Example
const data = new Uint8Array([72, 101, 108, 108, 111]); // "Hello"
const base32Encoded = encodeBase32Uppercase(data);
console.log(base32Encoded); // "JBSWY3DP"
const base32Decoded = decodeBase32(base32Encoded);
console.log(new TextDecoder().decode(new Uint8Array(base32Decoded))); // "Hello"

// Base64 Example
const base64Encoded = encodeBase64Standard(data);
console.log(base64Encoded); // "SGVsbG8="
const base64Decoded = decodeBase64(base64Encoded);
console.log(new TextDecoder().decode(new Uint8Array(base64Decoded))); // "Hello"

// Hex Example
const hexEncoded = encodeHexUpperCase('Hello');
console.log(hexEncoded); // "48656C6C6F"
const hexDecoded = decodeHex(hexEncoded);
console.log(hexDecoded); // "Hello"
```

## Performance

This library is implemented in Rust and exposed to Node.js through NAPI-RS, providing significant performance benefits compared to pure JavaScript implementations. The native code execution ensures:

- Minimal memory overhead
- Fast encoding/decoding operations
- Efficient handling of large data sets

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - see the [LICENSE](LICENSE) file for details.

## Credits

Built with [NAPI-RS](https://napi.rs/)
