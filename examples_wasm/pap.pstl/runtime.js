const imports = require('../runtime_util')
const {setupFramework} = imports;
const fs = require('fs');
const { performance } = require('perf_hooks');

const memory = new WebAssembly.Memory({
    initial: 10,
    maximum: 100,
});
  
const fichier = "pap.wasm";

const wasmBuffer = fs.readFileSync(fichier);
WebAssembly.instantiate(wasmBuffer, {
    js: { mem: memory },
}).then((wasmModule) => {


    const { getpap, main, notpap, papcall, mainpap, mainpap2, __nb_args, __init_memory } = wasmModule.instance.exports;
    const {resetMem, interprete, mem} = setupFramework(__init_memory, __nb_args, memory)

    var startTime = performance.now();
    var loc = main();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(loc, deltaTime)
    
    // Réinitialise la mémoire
    resetMem()

    var startTime = performance.now();
    var loc = mainpap();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(loc, deltaTime)
    
    // Réinitialise la mémoire
    resetMem()
    
    var startTime = performance.now();
    var loc = mainpap2();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(loc, deltaTime)
    
    // Réinitialise la mémoire
    resetMem()
});