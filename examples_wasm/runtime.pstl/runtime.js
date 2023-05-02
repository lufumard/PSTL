const fs = require('fs');

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
    mem[loc] = CONST_CONTRUCTEURS.num;
    mem[loc+1] = 1; //une ref
    mem[loc+2] = loc1;
    mem[loc+3] = loc2;
    mem[0] += 4 * 4;
    return loc;
}

/**
 * loc : Loc
 * mem : Uint32Array
 * return void
 */ 
const interprete = (loc, mem) => {
    console.log("Mémoire :", mem)
    console.log("Nombre d'allocations : ", mem[0]/4);

    const interprete_rec = (loc, mem) => {
        console.log("Résultat : ")
        let type = mem[loc];
        let refs = mem[loc+1];
        switch (type) {
            case CONST_CONTRUCTEURS.false:
                return console.log("Loc : ", loc, "; refs :", refs, "; valeur : False")
            case CONST_CONTRUCTEURS.true:
                return console.log("Loc : ", loc, "; refs :", refs, "; valeur : True")
            case CONST_CONTRUCTEURS.nil:
                return console.log("Loc : ", loc, "; refs :", refs, "; valeur : Nil")
            case CONST_CONTRUCTEURS.num:
                let num = mem[loc+2];
                return console.log("Loc : ", loc, "; refs :", refs, "; valeur : Num of ", num)
            case CONST_CONTRUCTEURS.list:
                let loc1 = mem[loc+2] / 4;
                let loc2 = mem[loc+3] / 4;
                console.log("Loc : ", loc, "; refs :", refs, "; valeur : List of ", loc1, loc2)
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

    //res : Loc
    let res = exported_func();
    let loc = res/4;

    interprete(loc, mem)
    
    // Réinitialise la mémoire
    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
    mem[0] = 4;
});