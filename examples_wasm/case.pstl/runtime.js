import {CONST_CONTRUCTEURS, createFalse, createTrue, createNil, createList, createNum, initMem, resetMem, interprete} from '../runtime_util'
const fs = require('fs');
const { performance } = require('perf_hooks');

const memory = new WebAssembly.Memory({
    initial: 10,
    maximum: 100,
});
  
const fichier = "case.wasm";


const wasmBuffer = fs.readFileSync(fichier);
WebAssembly.instantiate(wasmBuffer, {
    js: { mem: memory },
}).then((wasmModule) => {

    // Initialisation de la mémoire
    var mem = initMem(memory);
    
    const { mainf, maint, __nb_args } = wasmModule.instance.exports;

    var startTime = performance.now();
    var loc = mainf();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(__nb_args, loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    resetMem(mem);

    var startTime = performance.now();
    var loc = maint();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(__nb_args, loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    resetMem(mem);
});