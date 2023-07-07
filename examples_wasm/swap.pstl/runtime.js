const imports = require('../runtime_util')
const {createList, createNum, setupFramework} = imports;
const fs = require('fs');
const { performance } = require('perf_hooks');

const memory = new WebAssembly.Memory({
    initial: 10,
    maximum: 100,
});
  
const fichier = "swap.wasm";

const wasmBuffer = fs.readFileSync(fichier);
WebAssembly.instantiate(wasmBuffer, {
    js: { mem: memory },
}).then((wasmModule) => {


    /**
     * Init memory
     */

    const { swap, __nb_args, __init_memory } = wasmModule.instance.exports;
    const {resetMem, interprete, mem} = setupFramework(__init_memory, __nb_args, memory)

    let a = createNum(1, mem);
    let b = createNum(2, mem);
    let l = createList(a, b, mem);

    /**
     * Execute function
     */

    

    interprete(l, deltaTime)

    var startTime = performance.now();
    var loc = swap(l);
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(loc, deltaTime)
    
    // Réinitialise la mémoire
    resetMem()
});