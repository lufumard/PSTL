const imports = require('../runtime_util')
const {setupFramework} = imports;

const fs = require('fs');
const { performance } = require('perf_hooks');

const memory = new WebAssembly.Memory({
    initial: 10,
    maximum: 100,
});
  
const fichier = "add.wasm";



const wasmBuffer = fs.readFileSync(fichier);
WebAssembly.instantiate(wasmBuffer, {
    js: { mem: memory },
}).then((wasmModule) => {

    const { add55, liste, __nb_args, __init_memory } = wasmModule.instance.exports;

    var { resetMem, interprete } = setupFramework(__init_memory, __nb_args, memory)

    var startTime = performance.now();
    var loc = add55();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(loc, deltaTime)

    // Réinitialise la mémoire
    resetMem();

    var startTime = performance.now();
    var loc = liste();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(loc, deltaTime)

    // Réinitialise la mémoire
    resetMem();
});