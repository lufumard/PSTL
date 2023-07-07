const imports = require('../runtime_util')
const {setupFramework} = imports;
const fs = require('fs');
const { performance } = require('perf_hooks');

const memory = new WebAssembly.Memory({
    initial: 10,
    maximum: 100,
});
  
const fichier = "liste.wasm";

const wasmBuffer = fs.readFileSync(fichier);
WebAssembly.instantiate(wasmBuffer, {
    js: { mem: memory },
}).then((wasmModule) => {



    const { liste, hl, liste1, head, tail, first, last, length, len_liste, len_liste1, papadd1, papbool, __nb_args, __init_memory } = wasmModule.instance.exports;
    const {resetMem, interprete, mem} = setupFramework(__init_memory, __nb_args, memory)

    console.log("\nList [1,2,3,4,5]")
    //res : Loc
    var startTime = performance.now();
    var loc = liste();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(loc, deltaTime)
    
    // Réinitialise la mémoire
    resetMem()

    console.log("\nList of 1")
    //res : Loc
    var startTime = performance.now();
    var loc = liste1();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(loc, deltaTime)
    
    // Réinitialise la mémoire
    resetMem()

    console.log("\nhead of List")
    //res : Loc
    var startTime = performance.now();
    var loc = head(liste());
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    

    interprete(loc, deltaTime)
    
    // Réinitialise la mémoire
    resetMem()

    console.log("\ntail of List")
    //res : Loc
    var startTime = performance.now();
    var loc = tail(liste());
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    

    interprete(loc, deltaTime)
    
    // Réinitialise la mémoire
    resetMem()

    console.log("\nlength of List")
    //res : Loc
    var startTime = performance.now();
    var loc = len_liste();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    

    interprete(loc, deltaTime)
    
    // Réinitialise la mémoire
    resetMem()

    console.log("\nlength of List1")
    //res : Loc
    var startTime = performance.now();
    var loc = len_liste1();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    

    interprete(loc, deltaTime)

    // Réinitialise la mémoire
    resetMem()

    console.log("\npapadd1 of List1 = [2, 2, 2, 2, 2, 2]")
    //res : Loc
    var startTime = performance.now();
    var loc = papadd1();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    

    interprete(loc, deltaTime)
    
    // Réinitialise la mémoire
    resetMem()
    
    console.log("\n papbool of [True, True, False] = [False, False, True]")
    //res : Loc
    var startTime = performance.now();
    var loc = papbool();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    

    interprete(loc, deltaTime)
    
    // Réinitialise la mémoire
    resetMem()

    console.log("\n head of [1, 2, 3, ...] = 1")
    //res : Loc
    var startTime = performance.now();
    var loc = hl();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    

    interprete(loc, deltaTime)
    
    // Réinitialise la mémoire
    resetMem()
});