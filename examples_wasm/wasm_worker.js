const fs = require('fs');
const importObject = {
  imports: {
    imported_func: arg => {
      console.log(arg);
    }
  }
};

const wasmBuffer = fs.readFileSync("./simple.wasm");
WebAssembly.instantiate(wasmBuffer, importObject).then(wasmModule => {
  // Exported function live under instance.exports
  wasmModule.instance.exports.exported_func();

});
