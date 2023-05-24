const fs = require('fs');
const { performance } = require('perf_hooks');

const memory = new WebAssembly.Memory({
    initial: 10,
    maximum: 100,
});
  
const fichier = "liste.wasm";

const CONST_CONTRUCTEURS = {
    false : 0,
    true  : 1,
    nil   : 2,
    list  : 3,
    num   : 4,
    pap   : 5
}

/**
 * type Loc = number
*/

/**
 * mem  : Uint32Array
 * return Loc
 */ 
const createFalse = (mem) => {
    let loc = mem[0];
    mem[loc] = CONST_CONTRUCTEURS.false;
    mem[loc+1] = 1; // une ref
    mem[0] += 2 * 4;
    return loc;
}

/**
 * mem  : Uint32Array
 * return Loc
 */ 
const createTrue = (mem) => {
    let loc = mem[0];
    mem[loc] = CONST_CONTRUCTEURS.true;
    mem[loc+1] = 1; // une ref
    mem[0] += 2 * 4;
    return loc;
}

/**
 * mem  : Uint32Array
 * return Loc
 */ 
const createNil = (mem) => {
    let loc = mem[0];
    mem[loc] = CONST_CONTRUCTEURS.nil;
    mem[loc+1] = 1; // une ref
    mem[0] += 2 * 4;
    return loc;
}

/**
 * num  : number
 * mem  : Uint32Array
 * return Loc
 */ 
const createNum = (num, mem) => {
    let loc = mem[0];
    mem[loc] = CONST_CONTRUCTEURS.num;
    mem[loc+1] = 1;
    mem[loc+2] = num;
    mem[0] += 3 * 4;
    return loc;
}

/**
 * loc1 : Loc
 * loc2 : Loc
 * mem  : Uint32Array
 * return Loc
 */ 
const createList = (loc1, loc2, mem) => {
    let loc = mem[0];
    mem[loc] = CONST_CONTRUCTEURS.list;
    mem[loc+1] = 1; //une ref
    mem[loc+2] = loc1;
    mem[loc+3] = loc2;
    mem[0] += 4 * 4;
    return loc;
}


const log_mem = (mem) => {
    console.log("Mémoire :", mem);
}

/**
 * loc : Loc
 * mem : Uint32Array
 * return void
 */ 
const interprete = (loc, mem, dt, __nb_args) => {
    console.log("Mémoire :", mem)
    var nb_alloc = 0;
    var i=1;
    var alive = 0;
    var obj_alive = 0;
    while(i<mem[0]/4){
        nb_alloc++;
        if (mem[i+1] != 0){
            if (mem[i+1] < 0){
                console.log(`refs neg @mem[${i}], type ${mem[i]} (refs=${mem[i+1]})`)
            } else {
                console.log(`refs pos @mem[${i}], type ${mem[i]} (refs=${mem[i+1]})`)
            }
            alive+=mem[i+1];
            obj_alive++;
        } 
        if (mem[i] <= CONST_CONTRUCTEURS.nil){

            i+=2;
        }else if (mem[i] == CONST_CONTRUCTEURS.num){

            i+=3;
        }else if (mem[i] == CONST_CONTRUCTEURS.list){
            
            i+= 4;
        }else{
            var id = mem[i+2];
            nb_args = __nb_args(id);
            i+= nb_args + 4;
        }
    }
    console.log("Nombre d'allocations : ", nb_alloc, `(${mem[0]/4} blocs alloués)`);
    console.log(`Objets en vie : ${obj_alive} (${alive} refs)`)
    console.log(`Résultat en ${dt} ms`)
    const interprete_rec = (loc, mem) => {
        let type = mem[loc];
        let refs = mem[loc+1];
        switch (type) {
            case CONST_CONTRUCTEURS.false:
                return console.log(`loc : @${loc*4}; refs : ${refs} ; valeur :`, false)
            case CONST_CONTRUCTEURS.true:
                return console.log(`loc : @${loc*4}; refs : ${refs} ; valeur :`, true)
            case CONST_CONTRUCTEURS.nil:
                return console.log(`loc : @${loc*4}; refs : ${refs} ; valeur : Nil`)
            case CONST_CONTRUCTEURS.num:
                let num = mem[loc+2];
                return console.log(`loc : @${loc*4}; refs : ${refs} ; valeur : Num of`, num)
            case CONST_CONTRUCTEURS.list:
                let loc1 = mem[loc+2] / 4;
                let loc2 = mem[loc+3] / 4;
                console.log(`loc : @${loc}; refs : ${refs} ; valeur : List of @${loc1*4} @${loc2*4}`)
                if(loc === loc1) console.log("Liste infinie !");
                else interprete_rec(loc1, mem)
                if(loc === loc2) console.log("Liste infinie !");
                else interprete_rec(loc2, mem)
                return
            default:
                return console.log("Loc : ", loc*4, "type inconnu :", type)
        }
    }

    return interprete_rec(loc, mem);
}


const wasmBuffer = fs.readFileSync(fichier);
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


    const { liste, hl, liste1, head, tail, first, last, length, len_liste, len_liste1, papadd1, papbool, __nb_args } = wasmModule.instance.exports;

    console.log("\nList [1,2,3,4,5]")
    //res : Loc
    var startTime = performance.now();
    var res = liste();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;

    interprete(loc, mem, deltaTime, __nb_args)
    
    // Réinitialise la mémoire
    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
    mem[0] = 4;

    console.log("\nList of 1")
    //res : Loc
    var startTime = performance.now();
    var res = liste1();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;

    interprete(loc, mem, deltaTime, __nb_args)
    
    // Réinitialise la mémoire
    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
    mem[0] = 4;

    console.log("\nhead of List")
    //res : Loc
    var startTime = performance.now();
    var res = head(liste());
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;

    interprete(loc, mem, deltaTime, __nb_args)
    
    // Réinitialise la mémoire
    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
    mem[0] = 4;

    console.log("\ntail of List")
    //res : Loc
    var startTime = performance.now();
    var res = tail(liste());
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;

    interprete(loc, mem, deltaTime, __nb_args)
    
    // Réinitialise la mémoire
    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
    mem[0] = 4;

    console.log("\nlength of List")
    //res : Loc
    var startTime = performance.now();
    var res = len_liste();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;

    interprete(loc, mem, deltaTime, __nb_args)
    
    // Réinitialise la mémoire
    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
    mem[0] = 4;

    console.log("\nlength of List1")
    //res : Loc
    var startTime = performance.now();
    var res = len_liste1();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;

    interprete(loc, mem, deltaTime, __nb_args)

    // Réinitialise la mémoire
    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
    mem[0] = 4;

    console.log("\npapadd1 of List1 = [2, 2, 2, 2, 2, 2]")
    //res : Loc
    var startTime = performance.now();
    var res = papadd1();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;

    interprete(loc, mem, deltaTime, __nb_args)
    
    // Réinitialise la mémoire
    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
    mem[0] = 4;
    
    console.log("\n papbool of [True, True, False] = [False, False, True]")
    //res : Loc
    var startTime = performance.now();
    var res = papbool();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;

    interprete(loc, mem, deltaTime, __nb_args)
    
    // Réinitialise la mémoire
    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
    mem[0] = 4;

    console.log("\n head of [1, 2, 3, ...] = 1")
    //res : Loc
    var startTime = performance.now();
    var res = hl();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;
    var loc = res/4;

    interprete(loc, mem, deltaTime, __nb_args)
    
    // Réinitialise la mémoire
    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
    mem[0] = 4;
});