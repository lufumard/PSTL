const imports = require('../runtime_util')
const {setupFramework} = imports;

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

    const { hauteur_test, somme_test, ephf, max, max_arbre, min, min_arbre, add_arbre,arbre_test, nb_noeuds_test, __nb_args, __init_memory } = wasmModule.instance.exports;

    const {resetMem, interprete, mem} = setupFramework(__init_memory, __nb_args, memory);

    console.log(mem);

    console.log("Somme = 13")
    var startTime = performance.now();
    var loc = somme_test();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(loc, deltaTime)
    
    // Réinitialise la mémoire
    resetMem();

    console.log("\nHauteur = 4")
    var startTime = performance.now();
    var loc = hauteur_test();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(loc, deltaTime)
    
    // Réinitialise la mémoire
    resetMem();

    console.log("\nMax = 4")
    var startTime = performance.now();
    var loc = max_arbre();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(loc, deltaTime)
    
    // Réinitialise la mémoire
    resetMem();

    console.log("\nMin = 1")
    var startTime = performance.now();
    var loc = min_arbre();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(loc, deltaTime)
    
    // Réinitialise la mémoire
    resetMem();
    
    console.log("\n\nNB_noeuds = 6")
    var startTime = performance.now();
    var loc = nb_noeuds_test();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(loc, deltaTime)
    
    // Réinitialise la mémoire
    resetMem();

    console.log("\n\nHauteur")
    var startTime = performance.now();
    var loc = hauteur_test();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(loc, deltaTime)
    
    // Réinitialise la mémoire
    resetMem();
    
    console.log("\n\nAdd 5")
    var startTime = performance.now();
    var loc = add_arbre();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(loc, deltaTime)
    
    // Réinitialise la mémoire
    resetMem();
});