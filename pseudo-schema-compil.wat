fichier (fonctions : Vec<Program>)

js:
const memory = new WebAssembly.Memory({
  initial: 10,
  maximum: 100,
});

WebAssembly.instantiateStreaming(fetch("memory.wasm"), {
  js: { mem: memory },
}).then((obj) => {
  const sum = obj.instance.exports.accumulate(0, 10);
  console.log(sum);
});

---------------------
(module
    (memory (import "js" "mem") 1)
compile_program(fonctions[i])*
)


compile_program (cste : Const, fun:Fn)
---------------------
nom = string_of_const(cste)
(func ${nom} (export "{nom}") 
compile_fn(fun)
)



compile_fn (nom : Const,  params :Vec<Var>, body : FnBody)
---------------------
for param in params {
(param $params[i] i32 )
}
(result i32)
;; les lignes au dessus sont en une ligne avec la signature de la fonction 
;; sous la forme (func $ajout (export "ajout") (param $a i32) (param $b i32) (result i32)
let vars : Vec<&String> = catch_var_names(body)
for s in vars {
  (local ${s} i32)
}
compile_fnbody(body)



compile_let (var:Var, expr: Expr, fnbody:FnBody)
---------------------
compile_expr(expr)
if expr == Ret(var) {
  return
} else {
  let v = string_of_var(var)
  local.set ${v}
  compile_fnbody(fnbody)
}


compile_return (var : Var)
---------------------
compile_var(var)
return


compile_inc (var:Var, fnbody:FnBody)
---------------------
compile_var(var) ;; @var
get_ref_loc(var) ;; @var @ref
i32.load         ;; @var #ref
i32.const 1      ;; @var #ref 1
i32.add          ;; @var #ref+1
call $__set_ref

compile_fnbody(fnbody)


compile_dec (var:Var, fnbody:FnBody)
---------------------
compile_dec_body(var)
compile_fnbody(fnbody)


compile_reset (var:Var)
---------------------
compile_var(var)
call $__reset

compile_dec_body(var:Var)
---------------------
compile_var(var) ;; @var
get_ref_loc(var) ;; @var @ref
i32.load         ;; @var #ref
i32.const 1      ;; @var #ref 1
i32.sub          ;; @var #ref-1
call $__set_ref


(func $__reset (param $var i32) (result i32)
  (local $__intern_var i32)
  compile_dec_body(Var("var"))
  get_ref_loc(Var("var"))
  i32.load

  i32.eqz
  if
    i32.const 0
    return
  end
  local.get $var
)


compile_reuse (var:Var, ctor: i32, args: Either<i32, Vec<Var>>)
---------------------
compile_var(var)
;; teste si la variables est @0 (null) ou pas
    

    
;; types égaux
compile_var(var)
i32.load
local.tee $__intern_var
i32.const {ctor}
i32.eq
if ctor.clone() <= 3 {
  ;; types tous les deux <= à 3
  local.get $__intern_var
  i32.const 3
  i32.le_s
  i32.or ;; type var <= 3 OU types égaux
  i32.and ;; ET @var != 0
} else {
  i32.and ;; types égaux ET @var != 0
}

;; si on peut réutiliser l'emplacement mémoire
if
  match ctor {
    CONST_NUM => {
      compile_reuse_no_arg(var, CONST_NUM)
      compile_var(var)
      i32.const 8
      i32.add
      i32.const {args.Left}
      i32.store
    },
    CONST_LIST => {
      compile_reuse_no_arg(var, CONST_LIST)
      compile_var(var)
      i32.const 8
      i32.add
      compile_var(args.Right[0])
      i32.store
      compile_var(var)
      i32.const 12
      i32.add
      compile_var(args.Right[1])
      i32.store
    },
    _ => compile_reuse_no_arg(var, ctor),
  }

;; sinon, si on doit reprendre un espace mémoire
else
  match ctor {
    CONST_FALSE => compile_make_false(),
    CONST_TRUE => compile_make_true(),
    CONST_NIL => compile_make_nil(),
    CONST_NUM => compile_make_num(args.Left),
    CONST_LIST => compile_make_list(args.Right[0], args.Right[1]),
  }
  ;; on n'oublie pas de remettre l'adresse dans la variable
  let v = string_of_var(var)
  local.set ${v}

;; if ne doit rien laisser de plus sur la pile
end
compile_var(var)


compile_reuse_no_arg (var:Var, ctor:i32)
---------------------
compile_var(var)
i32.const {ctor}
i32.store
compile_var(var)
i32.const 1
call $__set_ref

get_ref_loc(var:Var)
---------------------
compile_var(var)
i32.const 4
i32.add

compile_var (var:Var)
---------------------
let s = string_of_var(var)
local.get ${s}


compile_value(n: i32)
---------------------
i32.const {n}
make_num()



compile_get_num (var:Var)
---------------------
let s = string_of_var(var)
local.get ${s}
i32.const 8 ;;décallage de deux entiers, la place de la valeur du nombre
i32.add
i32.load

compile_get_bool (var:Var)
---------------------
  compile_var(var)
i32.load ;; types : 0=FALSE, 1=TRUE, 2=NIL, ... donc la valeur du booleen est son type

compile_add (vars:Vec!<Var>)
---------------------
compile_get_num(vars[0])
compile_get_num(vars[1])
i32.add
compile_make_num()
;; valeur en haut de la pile : l'adresse de l'objet

compile_sub (vars:Vec!<Var>)
---------------------
compile_get_num(vars[0])
compile_get_num(vars[1])
i32.sub
compile_make_num()
;; valeur en haut de la pile : l'adresse de l'objet
...

compile_and (vars:Vec!<Var>)
---------------------
compile_get_bool(vars[0]);
(if (then
compile_get_bool(vars[1])
(if (then
compile_make_true()
) (else
compile_make_false()
))) (else
compile_make_false()
))
;; valeur en haut de la pile : l'adresse de l'objet

compile_or (vars:Vec!<Var>)
---------------------
compile_get_bool(vars[0])
(if (then
compile_make_true()
) (else
compile_get_bool(vars[1])
(if (then
compile_make_true()
) (else
compile_make_false()
))))
;; valeur en haut de la pile : l'adresse de l'objet

compile_not (var:Var)
---------------------
compile_get_bool(var)
(if (then
compile_make_false()
) (else
compile_make_true()
))
;; valeur en haut de la pile : l'adresse de l'objet


crée un constructeur sans argument en wat
---------------------
(func $__make_no_arg (param $b i32) (result i32)
  ;; true ou false ou nil
  local.get $b
  call $__init_type

  ;; références
  i32.const 0 ;; 0
  i32.load    ;; x
  i32.const 1 ;; x 1
  call $__set_ref

  ;; préparation de la valeur de retour
  i32.const 0 ;; 0
  i32.load    ;; x

  ;; mise à jour de memory[0]
  i32.const 8         ;; x 8
  call $__offset_next ;; x
  
  ;; la valeur en haut de la pile : x
)


(func $__init_type (param $t i32)
  i32.const 0 ;; 0
  i32.load    ;; x
  local.get $t;; x t
  i32.store   ;; 
)

(func $__offset_next (param $n i32)
;; mise à jour de memory[0]
  i32.const 0 ;; 0
  i32.const 0 ;; 0 0
  i32.load    ;; 0 x
  local.get $n;; 0 x n
  i32.add     ;; x 0 (x+n)
  i32.store   ;;
)

(func $__set_ref (param $adr i32) (param $ref i32)
  ;; mise à jour de memory[0]
  local.get $adr ;; @x
  i32.const 4    ;; @x 4
  i32.add        ;; @refs
  local.get $n   ;; @refs n
  i32.store      ;;
)

crée un constructeur de nombre en wat
---------------------
(func $__make_num (param $n i32) (result i32)
  ;; stoque le type du constructeur
  i32.const {CONST_NUM} ;; 4
  call $__init_type

  ;; références
  i32.const 0 ;; 0
  i32.load    ;; x
  i32.const 1 ;; x 1
  call $__set_ref
  
  ;; stoque le nombre
  i32.const 0 ;; 0
  i32.load    ;; x
  i32.const 8 ;; x 
  i32.add     ;; (x+8)
  local.get $n;; (x+8) n
  i32.store   ;;

  ;; préparation de la valeur de retour
  i32.const 0 ;; 0
  i32.load    ;; x

  ;; mise à jour de memory[0]
  i32.const 12     ;; x 12
  call $__offset_next ;; x

  ;; la valeur en haut de la pile : x
)


(func $__make_list (param $a i32) (param $b i32) (result i32)
  ;; stoque le type du constructeur
  i32.const {CONST_LIST} ;; 3
  call $__init_type

  ;; références
  i32.const 0 ;; 0
  i32.load    ;; x
  i32.const 1 ;; x 1
  call $__set_ref
  
  ;; stoque la première adresse
  i32.const 0 ;; 0
  i32.load    ;; x
  i32.const 8 ;; x 8
  i32.add     ;; (x+8)
  local.get $a;; (x+8) a
  i32.store   ;;

  ;; stoque la deuxième adresse
  i32.const 0 ;; 0
  i32.load    ;; x
  i32.const 12;; x 12
  i32.add     ;; (x+12)
  local.get $b;; (x+12) b
  i32.store   ;;

  ;; préparation de la valeur de retour
  i32.const 0 ;; 0
  i32.load    ;; x

  ;; mise à jour de memory[0]
  i32.const 16     ;; x 16
  call $__offset_next ;; x

  ;; la valeur en haut de la pile : x
)


(func $__make_pap (param $id i32) (result i32)
  ;; stoque le type du constructeur
  i32.const {CONST_PAP} ;; 5
  call $__init_type

  ;; références
  i32.const 0 ;; 0
  i32.load    ;; x
  i32.const 1 ;; x 1
  call $__set_ref
  
  ;; stoque l'id de la fonction
  i32.const 0 ;; 0
  i32.load    ;; x
  i32.const 8 ;; x 8
  i32.add     ;; (x+8)
  local.get $id;; (x+8) id
  i32.store   ;;

  ;; stoque le nombre d'arguments
  i32.const 0 ;; 0
  i32.load    ;; x
  i32.const 12;; x 12
  i32.add     ;; (x+12)
  i32.const 0 ;; (x+12) 0
  i32.store   ;;

  ;; préparation de la valeur de retour
  i32.const 0 ;; 0
  i32.load    ;; x

  ;; mise à jour de memory[0]
  i32.const 16     ;; x 16
  local.get $id    ;; x 16 id
  call $__nb_args  ;; x 16 nb_args
  i32.add          ;; x offset
  call $__offset_next ;; x

  ;; la valeur en haut de la pile : x
)

(func $__nb_args (param $id i32) (result i32)
  for n in 0..fn_desc.len() {
    ;; on crée un block pour chaque cas énuméré
    (block $__case{i}
  }
  ;; on charge le type de la variable
  local.get $id
  ;; br_table choisi un enbranchement selon la valeur du type de la variable
  ;; br renvoie à la fin du block indiqué, 
  ;; donc si on veut éxécuter la suite du code de block $__case1, il faut faire br $__case2
  (br_table 
  for i in 0..fn_desc.len() {
    $__case{len-1-i}
  }
  )
  for (_, desc) in fn_desc {
    )
    i32.const {desc.nb_args}
    return
  }
)

(func $__exec_pap (param $pap i32) (result i32)
  (local $p_0 i32)
  (local $p_1 i32)
  for i in 0..fn_desc.len() {
    ;; on crée un block pour chaque cas énuméré
    (block $__case{i}
  }
  ;; on charge le type de la variable
  get_pap_id(Var("pap"))
  ;; br_table choisi un enbranchement selon la valeur du type de la variable
  ;; br renvoie à la fin du block indiqué, 
  ;; donc si on veut éxécuter la suite du code de block $__case1, il faut faire br $__case2
  (br_table 
  for n in 0..fn_desc.len() {
    $__case{len-1-i}
  }
  )
  for (_, desc) in fn_desc {
    )
    if is_primitive(&desc.name){
      for i in 0..desc.nb_args {
        (i32.add (local.get $pap) (i32.const {16+i*4}))
        i32.load
        
        local.set $p_{i}
      }

      let vars = vec![Var("p_0"), Var("p_1")];
      compile_fncall_primitive(desc.name.clone(), vars);
    } else {
      for i in 0..desc.nb_args {
        (i32.add (local.get $pap) (i32.const {16+i*4}))
        i32.load
      }
      call $fun_{desc.name}
    }
    return
  }
)

(func $__copy_pap (param $var i32) (result i32)
  (local $pap i32)
  (local $args_rest i32)
  (local $loc_arg_var i32)
  (local $loc_arg_pap i32)
  ;; make new pap
  (i32.add (local.get $var) (i32.const 8))
  call $__make_pap
  
  local.set $pap

  ;; copy nb_args
  (i32.add (local.get $pap) (i32.const 12))
  (i32.add (local.get $var) (i32.const 12))  
  i32.load
  local.tee $args_rest


  i32.store

  ;; copy args
  (i32.add (local.get $var) (i32.const 16))
  local.set $loc_arg_var
  
  (i32.add (local.get $pap) (i32.const 16))  
  local.set $loc_arg_pap

  (loop $set_arg
    local.get $loc_arg_pap
    local.get $loc_arg_var
    i32.store

    (i32.add (local.get $loc_arg_pap) (i32.const 4))    
    local.set $loc_arg_pap
    
    (i32.add (local.get $loc_arg_var) (i32.const 4))
    local.set $loc_arg_var

    (i32.sub (local.get $args_rest) (i32.const 1))
    local.tee $args_rest
    br_if $set_arg
  )
  local.get $pap
)




compile_make_false
---------------------
i32.const {CONST_FALSE}
call $__make_no_arg

compile_make_true
---------------------
i32.const {CONST_TRUE}
call $__make_no_arg

compile_make_nil
---------------------
i32.const {CONST_NIL}
call $__make_no_arg

compile_make_num
;; pre : le nombre à créer est en haut de la pile
---------------------
call $__make_num

compile_make_list
;; pre : les arguments sont en haut de la pile
---------------------
call $__make_list

compile_case (var:Var, bodys:Vec<FnBody>)
---------------------
for n in 0..bodys.len() {
  ;; on crée un block pour chaque cas énuméré
  (block $__case{i}
}
;; on charge le type de la variable
compile_var(var)
i32.load
;; br_table choisi un enbranchement selon la valeur du type de la variable
;; br renvoie à la fin du block indiqué, 
;; donc si on veut éxécuter la suite du code de block $__case1, il faut faire br $__case2
(br_table 
for n in 0..bodys.len() {
  $__case{len-1-i}
}
)
for body in bodys {
)
compile_fnbody(body)
}
 
