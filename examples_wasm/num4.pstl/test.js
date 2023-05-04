const fs = require('fs');

const memory = new WebAssembly.Memory({
    initial: 10,
    maximum: 100,
});

const wasmBuffer = fs.readFileSync('num4.wasm');
WebAssembly.instantiate(wasmBuffer, {js: { mem: memory }}).then(wasmModule => {
  // Exported function live under instance.exports
  const { num } = wasmModule.instance.exports;
  const sum = num();
  console.log(sum); // Outputs: 11 + 3 = 14
});