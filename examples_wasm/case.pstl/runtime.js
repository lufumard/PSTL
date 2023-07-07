const fs = require('fs');
const { performance } = require('perf_hooks');
const {setupFramework} = require('../runtime_util')

const memory = new WebAssembly.Memory({
    initial: 10,
    maximum: 100,
});
  
const fichier = "case.wasm";


const wasmBuffer = fs.readFileSync(fichier);
WebAssembly.instantiate(wasmBuffer, {
    js: { mem: memory },
}).then((wasmModule) => {

    const { mainf, maint, __nb_args, __init_memory } = wasmModule.instance.exports;
    const {resetMem, interprete} = setupFramework(__init_memory, __nb_args, memory)

    var startTime = performance.now();
    var loc = mainf();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(loc, deltaTime)
    
    // Réinitialise la mémoire
    resetMem();

    var startTime = performance.now();
    var loc = maint();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(loc, deltaTime)
    
    // Réinitialise la mémoire
    resetMem();
});