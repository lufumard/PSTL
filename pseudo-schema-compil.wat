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
compile_fonction(fun)
)



compile_fonction (nom : Const,  params :Vec<Var>, body : FnBody)
---------------------
for param in params {
(param $params[i] i32 )
}
(result i32)
;; les lignes au dessus sont en une ligne avec la signature de la fonction 
;; sous la forme (func $ajout (export "ajout") (param $a i32) (param $b i32) (result i32)
let vars = catch_vars(body)
for v in vars {
	compile_init_var(v)
}
compile_fnbody(body)


compile_init_var (var : Var)
---------------------
let s = string_of_var(var)
(local ${s} i32)


compile_let (var:Var, expr: Expr, fnbody:FnBody)
---------------------
compile_expr(expr)
let v = string_of_var(var)
local.set ${v}
compile_fnbody(fnbody)


compile_return (var : Var)
---------------------
compile_var(var)
return


compile_inc (var:Var, fnbody:FnBody)
---------------------
TODO
compile_fnbody(fnbody)


compile_dec (var:Var, fnbody:FnBody)
---------------------
TODO
compile_fnbody(fnbody)


compile_var (var:Var)
---------------------
let s = string_of_var(var)
local.get ${s}


compile_value(n: i32)
---------------------
i32.const {n}
make_num()


compile_ctor (i: int, params : Vec<Loc>)
---------------------


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
(block $__choice
;; on charge le type de la variable
compile_var(var)
i32.load
;; br_table choisi un enbranchement selon la valeur du type de la variable
;; br renvoie à la fin du block indiqué, 
;; donc si on veut éxécuter la suite du code de block $__case1, il faut faire br $__case2
(br_table 
for n in 1..bodys.len() {
  $__case{i}
}
$__choice)
)
for body in bodys {
compile_fnbody(body)
)
}
 
