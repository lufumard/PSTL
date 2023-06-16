const fs = require('fs');
const { performance } = require('perf_hooks');

const imports = require('runtime_util');
const {CONST_CONTRUCTEURS, createFalse, createTrue, createNil, createList, createNum, initMem, resetMem, interprete} = imports;

const memory = new WebAssembly.Memory({
    initial: 10,
    maximum: 100,
});


const wasmBuffer = fs.readFileSync("arbre.wasm");
WebAssembly.instantiate(wasmBuffer, {
    js: { mem: memory },
}).then((wasmModule) => {
    // Initialisation de la mémoire
    var mem = initMem(memory);

    /**
     * Execute function
     */


    const { exported_func, __nb_args } = wasmModule.instance.exports;

    var startTime = performance.now();
    var loc = exported_func();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(__nb_args, loc, mem, deltaTime)
    
    resetMem(mem);
});