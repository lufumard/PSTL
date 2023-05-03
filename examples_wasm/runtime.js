const fs = require('fs');
const { performance } = require('perf_hooks');

const memory = new WebAssembly.Memory({
    initial: 10,
    maximum: 100,
});
  
const fichier = "fichier.wasm";

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
    mem[loc] = CONST_CONTRUCTEURS.list;
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
    console.log("Mémoire :", mem)
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
    const interprete_rec = (l, mem) => {
        let loc = l / 4;
        let type = mem[loc];
        let refs = mem[loc+1];
        switch (type) {
            case CONST_CONTRUCTEURS.false:
                return console.log(`loc : @${l}; refs : ${refs} ; valeur :`, false)
            case CONST_CONTRUCTEURS.true:
                return console.log(`loc : @${l}; refs : ${refs} ; valeur :`, true)
            case CONST_CONTRUCTEURS.nil:
                return console.log(`loc : @${l}; refs : ${refs} ; valeur : Nil`)
            case CONST_CONTRUCTEURS.num:
                let num = mem[loc+2];
                return console.log(`loc : @${l}; refs : ${refs} ; valeur : Num of`, num)
            case CONST_CONTRUCTEURS.list:
                let loc1 = mem[loc+2];
                let loc2 = mem[loc+3];
                console.log(`loc : @${l}; refs : ${refs} ; valeur : List of @${loc1} @${loc2}`)
                if(l === loc1) console.log("Liste infinie !");
                else interprete_rec(loc1, mem)
                if(l === loc2) console.log("Liste infinie !");
                else interprete_rec(loc2, mem)
                return
            default:
                return console.log("Loc : ", l, "type inconnu :", type)
        }
    }

    return interprete_rec(loc, mem);
}


const wasmBuffer = fs.readFileSync(fichier);
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

    

    const { exported_func } = wasmModule.instance.exports;

    var startTime = performance.now();
    var loc = exported_func();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
    mem[0] = 4;
});