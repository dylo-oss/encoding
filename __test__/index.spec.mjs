import test from 'ava';

import { decodeHex, encodeHexLowerCase, encodeHexUpperCase } from '../index.js';

import {
  encodeBase32Lowercase,
  encodeBase32LowercaseNoPadding,
  encodeBase32Uppercase,
  encodeBase32UppercaseNoPadding,
  decodeBase32,
  decodeBase32IgnorePadding,
} from '../index.js';

import {
  encodeBase64Standard,
  encodeBase64Url,
  encodeBase64StandardNoPadding,
  encodeBase64UrlNoPadding,
  decodeBase64,
  decodeBase64IgnorePadding,
} from '../index.js';

//--------------------------------------------------------------------------------------------
test('decodeHex converts hex to string', (t) => {
  const hex = '68656c6c6f';
  const result = decodeHex(hex);
  t.is(result, 'hello');
});

test('encodeHexUpperCase converts hex to uppercase', (t) => {
  const hex = '68656c6c6f';
  const result = encodeHexUpperCase(hex);
  t.is(result, '68656C6C6F');
});

test('encodeHexLowerCase converts hex to lowercase', (t) => {
  const hex = '68656C6C6F';
  const result = encodeHexLowerCase(hex);
  t.is(result, '68656c6c6f');
});

test('decodeHex throws error for invalid hex', (t) => {
  const invalidHex = '68656c6c6';
  const error = t.throws(
    () => {
      decodeHex(invalidHex);
    },
    { instanceOf: Error }
  );

  t.is(error.message, 'Invalid hex string');
});

//--------------------------------------------------------------------------------------------------
const inputData = new Uint8Array([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);

const encodedUppercase = 'AAAQEAYEAUDAOCAJBIFQYDIOB4======';
const encodedLowercase = 'aaaqeayeaudaocajbifqydiob4======';
const encodedUppercaseNoPadding = 'AAAQEAYEAUDAOCAJBIFQYDIOB4';
const encodedLowercaseNoPadding = 'aaaqeayeaudaocajbifqydiob4';

test('encodeBase32Uppercase encodes correctly', (t) => {
  const result = encodeBase32Uppercase(inputData);
  t.is(result, encodedUppercase);
});

test('encodeBase32Lowercase encodes correctly', (t) => {
  const result = encodeBase32Lowercase(inputData);
  t.is(result, encodedLowercase);
});

test('encodeBase32LowercaseNoPadding encodes correctly', (t) => {
  const result = encodeBase32LowercaseNoPadding(inputData);
  t.is(result, encodedLowercaseNoPadding);
});

test('encodeBase32UppercaseNoPadding encodes correctly', (t) => {
  const result = encodeBase32UppercaseNoPadding(inputData);
  t.is(result, encodedUppercaseNoPadding);
});

const validBase32Input = encodedUppercase;
const validBase32InputNoPadding = encodedUppercaseNoPadding;

const expectedDecodedOutput = Array.from(inputData);

test('decodeBase32 decodes correctly', (t) => {
  const result = decodeBase32(validBase32Input);
  t.deepEqual(result, expectedDecodedOutput);
});

test('decodeBase32IgnorePadding decodes correctly', (t) => {
  const result = decodeBase32IgnorePadding(validBase32InputNoPadding);
  t.deepEqual(result, expectedDecodedOutput);
});

//------------------------------------------------------------------------------------------------------
function strToUint8Array(str) {
  return new Uint8Array(str.split('').map((c) => c.charCodeAt(0)));
}

test('encodeBase64Standard - basic encoding', (t) => {
  t.is(encodeBase64Standard(strToUint8Array('Hello')), 'SGVsbG8=');
  t.is(encodeBase64Standard(strToUint8Array('Hello World')), 'SGVsbG8gV29ybGQ=');
  t.is(encodeBase64Standard(new Uint8Array([])), '');
});

test('encodeBase64Standard - handles padding correctly', (t) => {
  t.is(encodeBase64Standard(strToUint8Array('a')), 'YQ==');
  t.is(encodeBase64Standard(strToUint8Array('ab')), 'YWI=');
  t.is(encodeBase64Standard(strToUint8Array('abc')), 'YWJj');
});

test('encodeBase64Url - basic encoding', (t) => {
  const input = strToUint8Array('Hello+World/123');
  t.is(encodeBase64Url(input), 'SGVsbG8rV29ybGQvMTIz');
  t.false(encodeBase64Url(input).includes('+'));
  t.false(encodeBase64Url(input).includes('/'));
});

test('encodeBase64StandardNoPadding - basic encoding', (t) => {
  t.is(encodeBase64StandardNoPadding(strToUint8Array('a')), 'YQ');
  t.is(encodeBase64StandardNoPadding(strToUint8Array('ab')), 'YWI');
  t.false(encodeBase64StandardNoPadding(strToUint8Array('abc')).includes('='));
});

test('encodeBase64UrlNoPadding - basic encoding', (t) => {
  const input = strToUint8Array('Hello+World/123');
  const encoded = encodeBase64UrlNoPadding(input);
  t.false(encoded.includes('+'));
  t.false(encoded.includes('/'));
  t.false(encoded.includes('='));
});

test('decodeBase64 - basic decoding', (t) => {
  t.deepEqual(decodeBase64('SGVsbG8='), Array.from(strToUint8Array('Hello')));
  t.deepEqual(decodeBase64('SGVsbG8gV29ybGQ='), Array.from(strToUint8Array('Hello World')));
});

test('decodeBase64 - handles different padding lengths', (t) => {
  t.deepEqual(decodeBase64('YQ=='), [97]);
  t.deepEqual(decodeBase64('YWI='), [97, 98]);
  t.deepEqual(decodeBase64('YWJj'), [97, 98, 99]);
});

test('decodeBase64IgnorePadding - basic decoding', (t) => {
  t.deepEqual(decodeBase64IgnorePadding('SGVsbG8'), Array.from(strToUint8Array('Hello')));
  t.deepEqual(decodeBase64IgnorePadding('SGVsbG8gV29ybGQ'), Array.from(strToUint8Array('Hello World')));
});

test('decodeBase64IgnorePadding - handles missing padding', (t) => {
  t.deepEqual(decodeBase64IgnorePadding('YQ'), [97]);
  t.deepEqual(decodeBase64IgnorePadding('YWI'), [97, 98]);
});

test('decodeBase64 - handles invalid input', (t) => {
  t.throws(() => decodeBase64('Invalid!Base64'));
  t.throws(() => decodeBase64('SGVs!bG8='));
});

test('all encoders - handle empty input', (t) => {
  const emptyInput = new Uint8Array([]);
  t.is(encodeBase64Standard(emptyInput), '');
  t.is(encodeBase64Url(emptyInput), '');
  t.is(encodeBase64StandardNoPadding(emptyInput), '');
  t.is(encodeBase64UrlNoPadding(emptyInput), '');
});

test('URL-safe encoding handles special characters correctly', (t) => {
  const input = new Uint8Array([251, 255, 191]);
  const encoded = encodeBase64Url(input);
  const standardEncoded = encodeBase64Standard(input);

  t.true(standardEncoded.includes('+') || standardEncoded.includes('/'));

  t.false(encoded.includes('+'));
  t.false(encoded.includes('/'));
  if (standardEncoded.includes('+')) {
    t.true(encoded.includes('-'));
  }
  if (standardEncoded.includes('/')) {
    t.true(encoded.includes('_'));
  }
});

test('encode-decode round trip', (t) => {
  const testCases = ['Hello', 'Hello World', '123!@#', ''];

  for (const testCase of testCases) {
    const input = strToUint8Array(testCase);

    t.deepEqual(decodeBase64(encodeBase64Standard(input)), Array.from(input));

    t.deepEqual(decodeBase64(encodeBase64Url(input)), Array.from(input));
  }
});
