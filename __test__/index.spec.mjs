import test from 'ava';

import { hexToString, stringToHex, toUppercaseHex, toLowercaseHex } from '../index.js';

test('hexToString converts hex to string', (t) => {
  const hex = '68656c6c6f';
  const result = hexToString(hex);
  t.is(result, 'hello');
});

test('stringToHex converts string to hex', (t) => {
  const string = 'hello';
  const result = stringToHex(string);
  t.is(result, '68656c6c6f');
});

test('toUppercaseHex converts hex to uppercase', (t) => {
  const hex = '68656c6c6f';
  const result = toUppercaseHex(hex);
  t.is(result, '68656C6C6F');
});

test('toLowercaseHex converts hex to lowercase', (t) => {
  const hex = '68656C6C6F';
  const result = toLowercaseHex(hex);
  t.is(result, '68656c6c6f');
});

test('hexToString throws error for invalid hex', (t) => {
  const invalidHex = '68656c6c6';
  const error = t.throws(
    () => {
      hexToString(invalidHex);
    },
    { instanceOf: Error }
  );

  t.is(error.message, 'Invalid hex string');
});
