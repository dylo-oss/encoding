// test.js

// Import the compiled Rust module
const { encodeHexLowercase, encodeHexUppercase, decodeHexString } = require('./index');

// Function to demonstrate encoding and decoding
function runTests() {
  // Test data for encoding
  const inputArray = new Uint8Array([255, 0, 127]);

  // Encoding to uppercase hex
  const upperHex = encodeHexUppercase(inputArray);
  console.log(`Uppercase Hex: ${upperHex}`);

  // Encoding to lowercase hex
  const lowerHex = encodeHexLowercase(inputArray);
  console.log(`Lowercase Hex: ${lowerHex}`);

  // Decoding from hex string
  const decodedArray = decodeHexString('FF007F');
  console.log(`Decoded Uint8Array: ${Array.from(decodedArray)}`); // Should print: [255, 0, 127]

  // Example of invalid input
  try {
    decodeHexString('GHI'); // Invalid characters
  } catch (error) {
    console.error(`Error: ${error.message}`); // Should handle and print the error
  }

  try {
    decodeHexString('FF0'); // Odd-length
  } catch (error) {
    console.error(`Error: ${error.message}`); // Should handle and print the error
  }
}

// Run the tests
runTests();
