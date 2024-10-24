import test from 'ava';

import { decodeHex, encodeHexLowerCase, encodeHexUpperCase } from '../index.js';

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
