const imports = require('../runtime_util')
const {createList, createNum, initMem, resetMem, interprete} = imports;
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

    // Initialisation de la mémoire
    const mem = initMem(memory)


    /**
     * Init memory
     */

    let a = createNum(1, mem);
    let b = createNum(2, mem);
    let l = createList(a, b, mem);

    /**
     * Execute function
     */

    const { swap, __nb_args } = wasmModule.instance.exports;

    interprete(__nb_args, l, mem, deltaTime)

    var startTime = performance.now();
    var loc = swap(l);
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(__nb_args, loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    resetMem(mem)
});