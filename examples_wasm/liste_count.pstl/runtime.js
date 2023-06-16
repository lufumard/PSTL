const imports = require('../runtime_util')
const {createFalse, createTrue, createNil, createList, createNum, initMem, resetMem, interprete} = imports;
const fs = require('fs');
const { performance } = require('perf_hooks');

const memory = new WebAssembly.Memory({
    initial: 10,
    maximum: 100,
});
  
const fichier = "liste_count.wasm";

const wasmBuffer = fs.readFileSync(fichier);
WebAssembly.instantiate(wasmBuffer, {
    js: { mem: memory },
}).then((wasmModule) => {

    // Initialisation de la mémoire
    const mem = initMem(memory);


    /**
     * Init memory
     */

    const objs = [
        createFalse(mem),
        createTrue(mem),
        createNil(mem),
        createList(0, 0, mem),
        createNum(0, mem)
    ]

    /**
     * Execute function
     */

    

    const { count, __nb_args } = wasmModule.instance.exports;
    for (ia=0; ia<objs.length; ia++){
        for (ib=0; ib<objs.length; ib++){
            let a = objs[ia];
            let b = objs[ib];
            console.log(a, b)
            var startTime = performance.now();
            var loc = count(a, b);
            var endTime = performance.now();
            var deltaTime = endTime - startTime;
            interprete(__nb_args, loc, mem, deltaTime)
        }
    }
    
    // Réinitialise la mémoire
    resetMem(mem)
});