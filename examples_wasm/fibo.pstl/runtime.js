const imports = require('../runtime_util')
const {createNum, initMem, resetMem, interprete} = imports;
const fs = require('fs');
const { performance } = require('perf_hooks');

const memory = new WebAssembly.Memory({
    initial: 100,
    maximum: 65536,
});

const wasmBuffer = fs.readFileSync("fibo.wasm");
WebAssembly.instantiate(wasmBuffer, {
    js: { mem: memory },
}).then((wasmModule) => {

    // Initialisation de la mémoire
    const mem = initMem(memory);

    const { fibo, main5, main10, main15, main20, main25, main30, main35, __nb_args } = wasmModule.instance.exports;
    
    console.log(`\n\n\nfibo_main.wasm fibo of 5`)

    var startTime = performance.now();
    var res = main5();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;

    console.log("Mémoire :", mem);
    
    interprete(__nb_args, loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    resetMem(mem);

    console.log(`\n\n\nfibo_main.wasm fibo of 10`)

    var startTime = performance.now();
    var res = main10();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;

    console.log("Mémoire :", mem);
    
    interprete(__nb_args, loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    resetMem(mem);
    console.log(`\n\n\nfibo_main.wasm fibo of 15`)

    var startTime = performance.now();
    var res = main15();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;

    console.log("Mémoire :", mem);
    
    interprete(__nb_args, loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    resetMem(mem);
    console.log(`\n\n\nfibo_main.wasm fibo of 20`)

    var startTime = performance.now();
    var res = main20();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;

    console.log("Mémoire :", mem);
    
    interprete(__nb_args, loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    resetMem(mem);
    console.log(`\n\n\nfibo_main.wasm fibo of 25`)

    var startTime = performance.now();
    //var res = fibo(createNum(2, mem));
    var res = main25();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;

    console.log("Mémoire :", mem);
    
    interprete(__nb_args, loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    for(i=1; i<= mem[0]/4; i++){mem[i]=0}

    mem[0] = 4;
    console.log(`\n\n\nfibo_main.wasm fibo of 30`)

    var startTime = performance.now();
    //var res = fibo(createNum(2, mem));
    var res = main30();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;

    console.log("Mémoire :", mem);
    
    interprete(__nb_args, loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    resetMem(mem);

    console.log(`\n\n\nfibo_main.wasm fibo of 2`)
    var startTime = performance.now();
    //var res = fibo(createNum(2, mem));
    var res = fibo(createNum(2, mem));
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;

    console.log("Mémoire :", mem);
    
    interprete(__nb_args, loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
});


WebAssembly.instantiate(fs.readFileSync("fibo_liste.wasm"), {
    js: { mem: memory },
}).then((wasmModule) => {

    // Initialisation de la mémoire

    /**
     * Init memory
     */



    /**
     * Execute function
     */

    const { fibo0, fibo1, fibo2, fibo20 } = wasmModule.instance.exports;
    const mem = new Int32Array(memory.buffer);
    mem[0] = 4;
    console.log(`\n\n\nfibo_liste.wasm fibo of 0`)

    var startTime = performance.now();
    var res = fibo0();
    var endTime = performance.now();""
    var deltaTime = endTime - startTime;
    var loc = res/4;
    
    interprete(__nb_args, loc, mem, deltaTime)

    resetMem(mem);
    console.log(`\n\n\nfibo_liste.wasm fibo of 1`)

    var startTime = performance.now();
    var res = fibo1();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;
    
    interprete(__nb_args, loc, mem, deltaTime)

    resetMem(mem);
    console.log(`\n\n\nfibo_liste.wasm fibo of 2`)

    var startTime = performance.now();
    var res = fibo2();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;
    
    interprete(__nb_args, loc, mem, deltaTime);

    resetMem(mem);
    console.log(`\n\n\nfibo_liste.wasm fibo of 20`)

    var startTime = performance.now();
    var res = fibo20();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;
    
    interprete(__nb_args, loc, mem, deltaTime);
});