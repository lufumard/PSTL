const fs = require('fs');
const { performance } = require('perf_hooks');

const memory = new WebAssembly.Memory({
    initial: 65536,
    maximum: 65536,
});
  
const fichier = "fibo_num.wasm";

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
    while(i<mem[0]/4){
        nb_alloc++;
        if (mem[i] <= CONST_CONTRUCTEURS.nil){
            i+=2;
        }else if (mem[i] == CONST_CONTRUCTEURS.num){
            i+=3;
        }else{
            i+=4;
        }
    }
    console.log("Nombre d'allocations : ", nb_alloc, `(${mem[0]/4} blocs alloués)`);
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
/*
const wasmBuffer = fs.readFileSync("fibo_main.wasm");
WebAssembly.instantiate(wasmBuffer, {
    js: { mem: memory },
}).then((wasmModule) => {

    // Initialisation de la mémoire
    const mem = new Uint32Array(memory.buffer);
    mem[0] = 4;


    /**
     * Init memory
     */



    /**
     * Execute function
     */

    
/*
    const { fibo } = wasmModule.instance.exports;

    for(n=10; n<= 50; n++){

        console.log(`\n\n\nfibo_main.wasm fibo of ${n} `)

        var nb = createNum(n, mem);
        var startTime = performance.now();
        var res = fibo(nb);
        var endTime = performance.now();
        var deltaTime = endTime - startTime;
        var loc = res/4;

        interprete(loc, mem, deltaTime)
        
        // Réinitialise la mémoire
        for(i=1; i<= mem[0]/4; i++){mem[i]=0}
        mem[0] = 4;
    }

});
*/
/*

const wasmBuffer1 = fs.readFileSync("fibo.wasm");
WebAssembly.instantiate(wasmBuffer1, {
    js: { mem: memory },
}).then((wasmModule) => {

    // Initialisation de la mémoire
    const mem = new Uint32Array(memory.buffer);
    mem[0] = 4;


    /**
     * Init memory
     */



    /**
     * Execute function
     */
/*
    console.log("\n\n\nfibo.wasm")

    const { fibo } = wasmModule.instance.exports;

    let n7 = createNum(7, mem);

    console.log("Mémoire :", mem);

    var startTime = performance.now();
    var res = fibo(n7);
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;

    interprete(loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
    mem[0] = 4;
});
*/


const wasmBuffer = fs.readFileSync("fibo_liste.wasm");
WebAssembly.instantiate(wasmBuffer, {
    js: { mem: memory },
}).then((wasmModule) => {

    // Initialisation de la mémoire
    const mem = new Uint32Array(memory.buffer);
    mem[0] = 4;


    /**
     * Init memory
     */



    /**
     * Execute function
     */

    

    const { fibo45 } = wasmModule.instance.exports;


    console.log(`\n\n\nfibo_main.wasm fibo of 10 `)

    //var nb = createNum(25, mem);
    var startTime = performance.now();
    var res = fibo45();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;

    interprete(loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
    mem[0] = 4;
    

});