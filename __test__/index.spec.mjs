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
// Sample input data for testing
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
