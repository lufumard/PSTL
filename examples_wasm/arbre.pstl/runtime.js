const imports = require('../runtime_util')
const {initMem, resetMem, interprete} = imports;

const fs = require('fs');
const { performance } = require('perf_hooks');

const memory = new WebAssembly.Memory({
    initial: 10,
    maximum: 100,
});


const wasmBuffer = fs.readFileSync("arbre.wasm");
WebAssembly.instantiate(wasmBuffer, {
    js: { mem: memory },
}).then((wasmModule) => {   

    const { hauteur_test, somme_test, ephf, max, max_arbre, min, min_arbre, add_arbre,arbre_test, nb_noeuds_test, __nb_args } = wasmModule.instance.exports;

    var mem = initMem(memory);

    console.log("Somme = 13")
    var startTime = performance.now();
    var loc = somme_test();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(__nb_args, loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    resetMem(mem);

    console.log("\nHauteur = 4")
    var startTime = performance.now();
    var loc = hauteur_test();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(__nb_args, loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    resetMem(mem);

    console.log("\nMax = 4")
    var startTime = performance.now();
    var loc = max_arbre();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(__nb_args, loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    resetMem(mem);

    console.log("\nMin = 1")
    var startTime = performance.now();
    var loc = min_arbre();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(__nb_args, loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    resetMem(mem);
    
    console.log("\n\nNB_noeuds = 6")
    var startTime = performance.now();
    var loc = nb_noeuds_test();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(__nb_args, loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    resetMem(mem);

    console.log("\n\nHauteur")
    var startTime = performance.now();
    var loc = hauteur_test();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(__nb_args, loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    resetMem(mem);
    
    console.log("\n\nAdd 5")
    var startTime = performance.now();
    var loc = add_arbre();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(__nb_args, loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    resetMem(mem);
});