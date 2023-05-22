const fs = require('fs');
const { performance } = require('perf_hooks');

const memory = new WebAssembly.Memory({
    initial: 65536,
    maximum: 65536,
});

const CONST_CONTRUCTEURS = {
    false : 0,
    true  : 1,
    nil   : 2,
    list  : 3,
    num   : 4
}

/**
 * type Loc = number
*/

/**
 * mem  : Uint32Array
 * return Loc
 */ 
const createFalse = (mem) => {
    let loc = mem[0]/4;
    mem[loc] = CONST_CONTRUCTEURS.false;
    mem[loc+1] = 1; // une ref
    mem[0] += 2 * 4;
    return loc*4;
}

/**
 * mem  : Uint32Array
 * return Loc
 */ 
const createTrue = (mem) => {
    let loc = mem[0]/4;
    mem[loc] = CONST_CONTRUCTEURS.true;
    mem[loc+1] = 1; // une ref
    mem[0] += 2 * 4;
    return loc*4;
}

/**
 * mem  : Uint32Array
 * return Loc
 */ 
const createNil = (mem) => {
    let loc = mem[0]/4;
    mem[loc] = CONST_CONTRUCTEURS.nil;
    mem[loc+1] = 1; // une ref
    mem[0] += 2 * 4;
    return loc*4;
}

/**
 * num  : number
 * mem  : Uint32Array
 * return Loc
 */ 
const createNum = (num, mem) => {
    let loc = mem[0]/4;
    mem[loc] = CONST_CONTRUCTEURS.num;
    mem[loc+1] = 1;
    mem[loc+2] = num;
    mem[0] += 3 * 4;
    return loc*4;
}

/**
 * loc1 : Loc
 * loc2 : Loc
 * mem  : Uint32Array
 * return Loc
 */ 
const createList = (loc1, loc2, mem) => {
    let loc = mem[0]/4;
    mem[loc] = CONST_CONTRUCTEURS.num;
    mem[loc+1] = 1; //une ref
    mem[loc+2] = loc1;
    mem[loc+3] = loc2;
    mem[0] += 4 * 4;
    return loc*4;
}

/**
 * loc : Loc
 * mem : Uint32Array
 * return void
 */ 
const interprete = (loc, mem, dt) => {
    var nb_alloc = 0;
    var i=1;
    var alive = 0;
    while(i<mem[0]/4){
        nb_alloc++;
        if (mem[i] <= CONST_CONTRUCTEURS.nil){
            if (mem[i+1] != 0) alive+=mem[i+1];
            i+=2;
        }else if (mem[i] == CONST_CONTRUCTEURS.num){
            if (mem[i+1] != 0) alive+=mem[i+1];
            i+=3;
        }else{
            if (mem[i+1] != 0) alive+=mem[i+1];
            i+=4;
        }
    }
    console.log("Nombre d'allocations : ", nb_alloc, `(${mem[0]/4} blocs alloués)`);
    console.log("En vie : ", alive);
    console.log(`Résultat en ${dt} ms`)
    const interprete_rec = (loc, mem) => {
        let type = mem[loc];
        let refs = mem[loc+1];
        switch (type) {
            case CONST_CONTRUCTEURS.false:
                return console.log(`loc : @${loc}; refs : ${refs} ; valeur :`, false)
            case CONST_CONTRUCTEURS.true:
                return console.log(`loc : @${loc}; refs : ${refs} ; valeur :`, true)
            case CONST_CONTRUCTEURS.nil:
                return console.log(`loc : @${loc}; refs : ${refs} ; valeur : Nil`)
            case CONST_CONTRUCTEURS.num:
                let num = mem[loc+2];
                return console.log(`loc : @${loc}; refs : ${refs} ; valeur : Num of`, num)
            case CONST_CONTRUCTEURS.list:
                let loc1 = mem[loc+2] / 4;
                let loc2 = mem[loc+3] / 4;
                console.log(`loc : @${loc}; refs : ${refs} ; valeur : List of @${loc1} @${loc2}`)
                if(loc === loc1) console.log("Liste infinie !");
                else interprete_rec(loc1, mem)
                if(loc === loc2) console.log("Liste infinie !");
                else interprete_rec(loc2, mem)
                return
            default:
                return console.log("Loc : ", loc, "type inconnu :", type)
        }
    }

    return interprete_rec(loc, mem);
}

const wasmBuffer = fs.readFileSync("fibo.wasm");
WebAssembly.instantiate(wasmBuffer, {
    js: { mem: memory },
}).then((wasmModule) => {

    // Initialisation de la mémoire
    const mem = new Int32Array(memory.buffer);
    mem[0] = 4;


    /**
     * Init memory
     */



    /**
     * Execute function
     */

    const { fibo, main5, main10, main15, main20, main25, main30, main35 } = wasmModule.instance.exports;
    
    console.log(`\n\n\nfibo_main.wasm fibo of 5`)

    var startTime = performance.now();
    var res = main5();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;

    console.log("Mémoire :", mem);
    
    interprete(loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
    mem[0] = 4;

    console.log(`\n\n\nfibo_main.wasm fibo of 10`)

    var startTime = performance.now();
    var res = main10();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;

    console.log("Mémoire :", mem);
    
    interprete(loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
    mem[0] = 4;
    console.log(`\n\n\nfibo_main.wasm fibo of 15`)

    var startTime = performance.now();
    var res = main15();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;

    console.log("Mémoire :", mem);
    
    interprete(loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
    mem[0] = 4;
    console.log(`\n\n\nfibo_main.wasm fibo of 20`)

    var startTime = performance.now();
    var res = main20();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;

    console.log("Mémoire :", mem);
    
    interprete(loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
    mem[0] = 4;
    console.log(`\n\n\nfibo_main.wasm fibo of 25`)

    var startTime = performance.now();
    //var res = fibo(createNum(2, mem));
    var res = main25();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;

    console.log("Mémoire :", mem);
    
    interprete(loc, mem, deltaTime)
    
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
    
    interprete(loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
    mem[0] = 4;

    console.log(`\n\n\nfibo_main.wasm fibo of 2`)
    var startTime = performance.now();
    //var res = fibo(createNum(2, mem));
    var res = fibo(createNum(2, mem));
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;

    console.log("Mémoire :", mem);
    
    interprete(loc, mem, deltaTime)
    
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
    
    interprete(loc, mem, deltaTime)
    console.log("Mémoire :", mem);    

    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
    mem[0] = 4;
    console.log(`\n\n\nfibo_liste.wasm fibo of 1`)

    var startTime = performance.now();
    var res = fibo1();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;
    
    interprete(loc, mem, deltaTime)
    console.log("Mémoire :", mem);

    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
    mem[0] = 4;
    console.log(`\n\n\nfibo_liste.wasm fibo of 2`)

    var startTime = performance.now();
    var res = fibo2();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;
    
    interprete(loc, mem, deltaTime);
    console.log("Mémoire :", mem);

    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
    mem[0] = 4;
    console.log(`\n\n\nfibo_liste.wasm fibo of 20`)

    var startTime = performance.now();
    var res = fibo20();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;
    
    interprete(loc, mem, deltaTime);
    console.log("Mémoire :", mem);
});