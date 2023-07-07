
const CONST_CONTRUCTEURS = {
    false : 0,
    true  : 1,
    nil   : 2,
    list  : 3,
    num   : 4,
    pap   : 5
}

const CONFIGURATION_TYPES = {
    i32 : "i32",
    i16 : "i16"
}

const ONLY_ONE_NO_ARG = true;

const getConfigurationTypes = () => CONFIGURATION_TYPES

/**
 * IMPORTANT : CONFIGURATION DES REFS
*/
const POSSIBLE_CONFIGURATION = {
    i16 : {
        getSize : () => CONFIGURATION_TYPES.i16,
        getType : (mem, loc) => (mem[loc] >> 16 ) & 0xFFFF,
        setType : (mem, loc, type) => mem[loc] = (type & 0xFFFF) << 16 + i16.getRefs(mem, loc),
        getRefs : (mem, loc) => mem[loc] & 0xFFFF,
        setRefs : (mem, loc, refs) => mem[loc] = i16.getType(mem, loc) << 16 + refs,
        getTypeRefsOffset : () => 1,
    },
    i32 : {
        getSize : () => CONFIGURATION_TYPES.i32,
        getType : (mem, loc) => (mem[loc]),
        setType : (mem, loc, type) => (mem[loc] = type),
        getRefs : (mem, loc) => (mem[loc+1]),
        setRefs : (mem, loc, refs) => (mem[loc+1] = refs),
        getTypeRefsOffset : () => 2,
    }
}

var CONFIGURATION = POSSIBLE_CONFIGURATION.i32;

const getSize = CONFIGURATION.getSize
const getType = CONFIGURATION.getType
const setType = CONFIGURATION.setType
const getRefs = CONFIGURATION.getRefs
const setRefs = CONFIGURATION.setRefs
const getTypeRefsOffset = CONFIGURATION.getTypeRefsOffset

// conf : string
const setConfiguration = (conf) => {
    switch (conf) {
        case CONFIGURATION_TYPES.i16:
            CONFIGURATION = POSSIBLE_CONFIGURATION.i16;    
            break;
        case CONFIGURATION_TYPES.i32:
            CONFIGURATION = POSSIBLE_CONFIGURATION.i32;    
            break;
        default:
            break;
    }
}

const getConfiguration = CONFIGURATION.getSize

/**
 * type Loc = number
*/

const newNoArg = (mem, n) => {
    let loc = mem[0]/4;
    setType(mem, loc, n)
    mem[0]+= 4*getTypeRefsOffset();
    return loc*4;
}

const createNoArg = (mem, n) => {
    let loc = (n+1)*getTypeRefsOffset();
    setRefs(mem, loc, 1)
    return loc*4;
}

/**
 * mem  : Uint32Array
 * return Loc
 */ 
const createFalse = (mem) => createNoArg(mem, CONST_CONTRUCTEURS.false)

/**
 * mem  : Uint32Array
 * return Loc
 */ 
const createTrue = (mem) => createNoArg(mem, CONST_CONTRUCTEURS.true)

/**
 * mem  : Uint32Array
 * return Loc
 */ 
const createNil = (mem) => createNoArg(mem, CONST_CONTRUCTEURS.nil)

/**
 * num  : number
 * mem  : Uint32Array
 * return Loc
 */ 
const createNum = (num, mem) => {
    let loc = mem[0]/4;
    setType(mem, loc, CONST_CONTRUCTEURS.num)
    setRefs(mem, loc, 1)
    mem[loc+getTypeRefsOffset()] = num;
    mem[0] += (1+getTypeRefsOffset()) * 4;
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
    setType(mem, loc, CONST_CONTRUCTEURS.list)
    setRefs(mem, loc, 1)
    mem[loc+CONFIGURATION.getSize()] = loc1;
    mem[loc+CONFIGURATION.getSize()+1] = loc2;
    mem[0] += (2 + CONFIGURATION.getSize()) * 4;
    return loc*4;
}

/**
 * __nb_args : (id_fun : Number) => Number
 * loc : Loc
 * mem : Uint32Array
 * delta_time : Number 
 * ?log_memory : Boolean
 * return void
 */ 
const interprete = (__nb_args, loc, mem, delta_time, log_memory=false) => {
    if (log_memory) console.log("Mémoire :", mem);
    var nb_alloc = 0;
    var i=1;
    var alive = 0;
    var obj_alive = 0;
    while(i<mem[0]/4){
        nb_alloc++;
        let type = getType(mem, i)
        let refs = getRefs(mem, i)
        if (refs != 0){
            if (refs < 0){
                console.log(`refs neg @mem[${i}], type ${type} (refs=${refs})`)
            } else {
                //console.log(`refs pos @mem[${i}], type ${type} (refs=${refs})`)
            }
            alive+=refs;
            obj_alive++;
        } 
        
        if (type <= CONST_CONTRUCTEURS.nil){
            i+= getTypeRefsOffset()
        }else if (type == CONST_CONTRUCTEURS.num){
            i+= getTypeRefsOffset() + 1
        }else if (type == CONST_CONTRUCTEURS.list){
            i+= getTypeRefsOffset() + 2
        }else{
            var id = mem[i+getTypeRefsOffset()];
            nb_args = __nb_args(id);
            i+= nb_args + 2 + getTypeRefsOffset();
        }
    }
    console.log("Nombre d'allocations : ", nb_alloc, `(${mem[0]/4} blocs alloués)`);
    console.log(`Objets en vie : ${obj_alive} (${alive} refs)`)
    console.log(`Résultat en ${delta_time} ms`)
    const interprete_rec = (l, mem) => {
        if (l == 0){
            return console.log(null)
        }
        let loc = l / 4;
        let type = getType(mem, loc)
        let refs = getRefs(mem, loc)
        switch (type) {
            case CONST_CONTRUCTEURS.false:
                return console.log(`loc : @${l}; refs : ${refs} ; valeur :`, false)
            case CONST_CONTRUCTEURS.true:
                return console.log(`loc : @${l}; refs : ${refs} ; valeur :`, true)
            case CONST_CONTRUCTEURS.nil:
                return console.log(`loc : @${l}; refs : ${refs} ; valeur : Nil`)
            case CONST_CONTRUCTEURS.num:
                let num = mem[loc+getTypeRefsOffset()];
                return console.log(`loc : @${l}; refs : ${refs} ; valeur : Num of`, num)
            case CONST_CONTRUCTEURS.list:
                let loc1 = mem[loc+getTypeRefsOffset()];
                let loc2 = mem[loc+getTypeRefsOffset()+1];
                console.log(`loc : @${l}; refs : ${refs} ; valeur : List of @${loc1} @${loc2}`)
                if(l === loc1) console.log("Liste infinie !");
                else interprete_rec(loc1, mem)
                if(l === loc2) console.log("Liste infinie !");
                else interprete_rec(loc2, mem)
                return
            case CONST_CONTRUCTEURS.pap:
                let id = mem[loc+getTypeRefsOffset()];
                let nb_args = mem[loc+getTypeRefsOffset()+1];
                console.log(`loc : @${l}; refs : ${refs} ; valeur : PAP of ${id}, ${nb_args} args`)
                for (i=0; i<nb_args; i++){
                    interprete_rec(mem[loc+i+2+getTypeRefsOffset()], mem)
                }
                return
            default:
                return console.log("Loc : ", l, "type inconnu :", type)
        }
    }

    return interprete_rec(loc, mem);
}

const initMem = (__init_memory, memory) => {
    const nmem = new Int32Array(memory.buffer);
    resetMem(__init_memory, nmem);
    return nmem
}

const resetMem = (__init_memory, mem) => {
    for(i=1; i<= mem[0]/4; i++){mem[i]=0}
    //mem[0] = 4;
    __init_memory()
}

const setupFramework = (__init_memory, __nb_args, memory) => {
    const nmem = initMem(__init_memory, memory)
    return {
        resetMem : () => resetMem(__init_memory, nmem),
        interprete : (loc, delta_time, log_memory=false) => interprete(__nb_args, loc, nmem, delta_time, log_memory),
        mem : nmem
    }
}

module.exports = {getConfiguration, getConfigurationTypes, setConfiguration, createFalse, createTrue, createNil, createList, createNum, setupFramework};