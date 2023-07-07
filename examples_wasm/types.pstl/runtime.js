const imports = require('../runtime_util')
const {setupFramework} = imports;
const fs = require('fs');
const { performance } = require('perf_hooks');

const memory = new WebAssembly.Memory({
    initial: 10,
    maximum: 100,
});
  
const fichier = "types.wasm";

const wasmBuffer = fs.readFileSync(fichier);
WebAssembly.instantiate(wasmBuffer, {
    js: { mem: memory },
}).then((wasmModule) => {

    
    const { num, mfalse, mtrue, nil, liste, __nb_args, __init_memory } = wasmModule.instance.exports;

    const {resetMem, interprete, mem} = setupFramework(__init_memory, __nb_args, memory);

    var startTime = performance.now();
    var loc = num();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    
    interprete(loc, deltaTime)
    
    // Réinitialise la mémoire
    resetMem()

    var startTime = performance.now();
    var loc = mfalse();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    
    interprete(loc, deltaTime)
    
    // Réinitialise la mémoire
    resetMem()

    var startTime = performance.now();
    var loc = mtrue();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    
    interprete(loc, deltaTime)
    
    // Réinitialise la mémoire
    resetMem()

    var startTime = performance.now();
    var loc = nil();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    
    interprete(loc, deltaTime)
    
    // Réinitialise la mémoire
    resetMem()

    var startTime = performance.now();
    var loc = liste();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    
    interprete(loc, deltaTime)
    
    // Réinitialise la mémoire
    resetMem()
});