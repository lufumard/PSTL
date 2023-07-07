const imports = require('../runtime_util')
const {setupFramework} = imports;
const fs = require('fs');
const { performance } = require('perf_hooks');

const memory = new WebAssembly.Memory({
    initial: 10,
    maximum: 100,
});
  
const fichier = "copy.wasm";

const wasmBuffer = fs.readFileSync(fichier);
WebAssembly.instantiate(wasmBuffer, {
    js: { mem: memory },
}).then((wasmModule) => {



    const { copy, main3, __nb_args, __init_memory } = wasmModule.instance.exports;
    const {resetMem, interprete} = setupFramework(__init_memory, __nb_args, memory)

    var startTime = performance.now();
    var loc = main3();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(loc, deltaTime)
    
    // Réinitialise la mémoire
    resetMem();
});