const fs = require('fs');
const { performance } = require('perf_hooks');

const imports = require('runtime_util');
const {CONST_CONTRUCTEURS, createFalse, createTrue, createNil, createList, createNum, setupFramework} = imports;

const memory = new WebAssembly.Memory({
    initial: 10,
    maximum: 100,
});


const wasmBuffer = fs.readFileSync("arbre.wasm");
WebAssembly.instantiate(wasmBuffer, {
    js: { mem: memory },
}).then((wasmModule) => {
    // Initialisation de la mémoire

    /**
     * Execute function
     */


    const { exported_func, __nb_args, __init_mem } = wasmModule.instance.exports;
    const {resetMem, interprete, mem} = setupFramework(__init_mem, __nb_args, memory);


    var startTime = performance.now();
    var loc = exported_func();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(loc, deltaTime)
    
    resetMem();
});