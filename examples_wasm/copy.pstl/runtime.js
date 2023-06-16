const imports = require('../runtime_util')
const {initMem, resetMem, interprete} = imports;
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

    // Initialisation de la mémoire
    var mem = initMem(memory);


    /**
     * Init memory
     */



    /**
     * Execute function
     */

    

    const { copy, main3, __nb_args } = wasmModule.instance.exports;

    var startTime = performance.now();
    var loc = main3();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(__nb_args, loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    resetMem(mem);
});