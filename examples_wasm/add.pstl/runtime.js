const imports = require('../runtime_util')
const {initMem, resetMem, interprete} = imports;

const fs = require('fs');
const { performance } = require('perf_hooks');

const memory = new WebAssembly.Memory({
    initial: 10,
    maximum: 100,
});
  
const fichier = "add.wasm";



const wasmBuffer = fs.readFileSync(fichier);
WebAssembly.instantiate(wasmBuffer, {
    js: { mem: memory },
}).then((wasmModule) => {

    var mem = initMem(memory);

    const { add55, liste, __nb_args } = wasmModule.instance.exports;


    var startTime = performance.now();
    var loc = add55();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(__nb_args, loc, mem, deltaTime)

    // Réinitialise la mémoire
    resetMem(mem);

    var startTime = performance.now();
    var loc = liste();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(__nb_args, loc, mem, deltaTime)

    // Réinitialise la mémoire
    resetMem(mem);
});