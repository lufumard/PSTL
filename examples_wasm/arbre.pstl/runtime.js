const fs = require('fs');
const { performance } = require('perf_hooks');

const memory = new WebAssembly.Memory({
    initial: 10,
    maximum: 100,
});

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



const wasmBuffer = fs.readFileSync("arbre.wasm");
WebAssembly.instantiate(wasmBuffer, {
    js: { mem: memory },
}).then((wasmModule) => {

    
    /**
     * loc : Loc
     * mem : Uint32Array
     * return void
     */ 
    const interprete = (loc, mem, dt) => {
        //console.log("Mémoire :", mem)
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
                    //console.log(`refs pos @mem[${i}], type ${mem[i]} (refs=${mem[i+1]})`)
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
        const interprete_rec = (l, mem) => {
            if (l == 0){
                return console.log(null)
            }
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
                case CONST_CONTRUCTEURS.pap:
                    let id = mem[loc+2];
                    let nb_args = mem[loc+3];
                    console.log(`loc : @${l}; refs : ${refs} ; valeur : PAP of ${id}, ${nb_args} args`)
                    for (i=0; i<nb_args; i++){
                        interprete_rec(mem[loc+i+4], mem)
                    }
                    return
                default:
                    return console.log("Loc : ", l, "type inconnu :", type)
            }
        }

        return interprete_rec(loc, mem);
    }

    // Initialisation de la mémoire
    const mem = new Int32Array(memory.buffer);
    mem[0] = 4;


    /**
     * Init memory
     */



    /**
     * Execute function
     */

    

    const { hauteur_test, somme_test, ephf, max, max_arbre, min, min_arbre, add_arbre,arbre_test, nb_noeuds_test, __nb_args } = wasmModule.instance.exports;

    console.log("Somme = 13")
    var startTime = performance.now();
    var loc = somme_test();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
    mem[0] = 4;
    console.log("\nHauteur = 4")
    var startTime = performance.now();
    var loc = hauteur_test();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
    mem[0] = 4;

    console.log("\nMax = 4")
    var startTime = performance.now();
    var loc = max_arbre();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
    mem[0] = 4;

    console.log("\nMin = 1")
    var startTime = performance.now();
    var loc = min_arbre();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
    mem[0] = 4;
    
    console.log("\n\nNB_noeuds = 6")
    var startTime = performance.now();
    var loc = nb_noeuds_test();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
    mem[0] = 4;

    console.log("\n\nHauteur")
    var startTime = performance.now();
    var loc = hauteur_test();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
    mem[0] = 4;

    console.log("\n\nAdd 5")
    var startTime = performance.now();
    var loc = add_arbre();
    var endTime = performance.now();
    var deltaTime = endTime - startTime;

    interprete(loc, mem, deltaTime)
    
    // Réinitialise la mémoire
    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
    mem[0] = 4;
});