const { decodeHex } = require('@dylo/encoding');

function decodeHexJS(hexString) {
  if (hexString.length % 2 !== 0) {
    throw new Error('Invalid hex string');
  }

  const byteArray = new Uint8Array(hexString.length / 2);

  for (let i = 0; i < hexString.length; i += 2) {
    byteArray[i / 2] = parseInt(hexString.substr(i, 2), 16);
  }

  return byteArray;
}

const warmup = (func, hex, warmupIterations) => {
  for (let i = 0; i < warmupIterations; i++) {
    func(hex);
  }
};

const benchmark = (func, hex, iterations) => {
  let totalDuration = 0;

  for (let i = 0; i < iterations; i++) {
    const start = performance.now();
    func(hex);
    const end = performance.now();
    totalDuration += end - start;
  }

  return totalDuration / iterations;
};

const large = '5468697320697320612074657374206d6573736167652e';
const small = '4d61';

const testCases = [
  { iterations: 1000000, warmup: 100, description: 'Small numbers, high iterations', hex: large },
  { iterations: 1000000, warmup: 100, description: 'Hundreds, high iterations', hex: small },
  { iterations: 10000, warmup: 10, description: 'Small numbers, low iterations', hex: large },
  { iterations: 10000, warmup: 10, description: 'Hundreds, low iterations', hex: small },
  { iterations: 1000000, warmup: 0, description: 'No warmup, small numbers', hex: large },
  { iterations: 1000000, warmup: 0, description: 'No warmup, hundreds', hex: small },
];

const results = [];

for (const { hex, iterations, warmup: warmupIterations, description } of testCases) {
  if (warmupIterations > 0) {
    warmup(decodeHex, hex, warmupIterations);
    warmup(decodeHexJS, hex, warmupIterations);
  }
  const nativeDuration = benchmark(decodeHex, hex, iterations);
  const packageDuration = benchmark(decodeHexJS, hex, iterations);
  results.push({
    input: `${hex}`,
    rust: nativeDuration,
    js: packageDuration,
    percentage: ((nativeDuration - packageDuration) / packageDuration) * 100,
    description,
  });
}

console.table(results, ['input', 'rust', 'js', 'percentage', 'description']);
