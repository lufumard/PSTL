(module
(memory (import "js" "mem") 1)
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
    ;; mise à jour des ref
    local.get $adr ;; @x
    i32.const 4    ;; @x 4
    i32.add        ;; @refs
    local.get $ref ;; @refs n
    i32.store      ;;
)
(func $__make_num (param $a i32) (result i32)
    ;; stoque le type du constructeur
    i32.const 4
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
    local.get $a;; (x+8) a
    i32.store   ;;
    ;; préparation de la valeur de retour
    i32.const 0 ;; 0
    i32.load    ;; x
    ;; mise à jour de memory[0]
    i32.const 12        ;; x 12
    call $__offset_next ;; x
)
(func $__make_list (param $a i32) (param $b i32) (result i32)
    ;; stoque le type du constructeur
    i32.const 3
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
    i32.const 16        ;; x 16
    call $__offset_next ;; x
)
(func $__reset (param $var i32) (result i32)
    local.get $var
    call $__dec
local.get $var
i32.const 4
i32.add
    i32.load
    i32.eqz
    if
        i32.const 0
        return
    end
    local.get $var
)
(func $__make_pap (param $a i32) (result i32)
    ;; stoque le type du constructeur
    i32.const 5
    call $__init_type
    ;; références
    i32.const 0 ;; 0
    i32.load    ;; x
    i32.const 1 ;; x 1
    call $__set_ref
    ;; stoque l'id de la fonction
    ;; stoque le nombre
    i32.const 0 ;; 0
    i32.load    ;; x
    i32.const 8 ;; x 
    i32.add     ;; (x+8)
    local.get $a;; (x+8) a
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
    i32.const 16        ;; x 16
    local.get $a        ;; x 16 a
    call $__nb_args     ;; x 16 nb_args
    i32.const 4         ;; x 16 nb_args 4
    i32.mul             ;; x 16 nb_args*4
    i32.add             ;; x offset
    call $__offset_next ;; x
)
(func $__nb_args (export "__nb_args") (param $id i32) (result i32)
    (block $__case0
    (block $__case1
    (block $__case2
    (block $__case3
    (block $__case4
    (block $__case5
    (block $__case6
    (block $__case7
    (block $__case8
    (block $__case9
    (block $__case10
    (block $__case11
    (block $__case12
    (block $__case13
    (block $__case14
    (block $__case15
    (block $__case16
    (block $__case17
    (block $__case18
    (block $__case19
    (block $__case20
    (block $__case21
    (block $__case22
    (block $__case23
    (block $__case24
    (block $__case25
    (block $__case26
    (block $__case27
    (block $__case28
    local.get $id
  (br_table 
$__case28 $__case27 $__case26 $__case25 $__case24 $__case23 $__case22 $__case21 $__case20 $__case19 $__case18 $__case17 $__case16 $__case15 $__case14 $__case13 $__case12 $__case11 $__case10 $__case9 $__case8 $__case7 $__case6 $__case5 $__case4 $__case3 $__case2 $__case1 $__case0 )
)
    i32.const 2 
    return
)
    i32.const 2 
    return
)
    i32.const 2 
    return
)
    i32.const 2 
    return
)
    i32.const 2 
    return
)
    i32.const 2 
    return
)
    i32.const 2 
    return
)
    i32.const 1 
    return
)
    i32.const 2 
    return
)
    i32.const 2 
    return
)
    i32.const 2 
    return
)
    i32.const 2 
    return
)
    i32.const 2 
    return
)
    i32.const 3 
    return
)
    i32.const 1 
    return
)
    i32.const 1 
    return
)
    i32.const 1 
    return
)
    i32.const 1 
    return
)
    i32.const 1 
    return
)
    i32.const 2 
    return
)
    i32.const 2 
    return
)
    i32.const 2 
    return
)
    i32.const 1 
    return
)
    i32.const 1 
    return
)
    i32.const 3 
    return
)
    i32.const 1 
    return
)
    i32.const 1 
    return
)
    i32.const 1 
    return
)
    i32.const 1 
    return
)
(func $__copy_pap (param $var i32) (result i32)
(local $pap i32)
(local $args_rest i32)
(local $loc_arg_var i32)
(local $loc_arg_pap i32)
(local $arg i32)
local.get $var
i32.const 8
i32.add
i32.load
call $__make_pap
local.set $pap
local.get $pap
i32.const 12
i32.add
local.get $var
i32.const 12
i32.add
i32.load
local.tee $args_rest
i32.store
local.get $var
i32.const 16
i32.add
local.set $loc_arg_var
local.get $pap
i32.const 16
i32.add
local.set $loc_arg_pap
local.get $args_rest
if
(loop $set_arg
    local.get $loc_arg_pap
    local.get $loc_arg_var
    i32.load
    i32.store
    local.get $loc_arg_var
    i32.load
    local.set $arg
local.get $arg
local.get $arg
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
    (i32.add (local.get $loc_arg_pap) (i32.const 4))
    local.set $loc_arg_pap
    (i32.add (local.get $loc_arg_var) (i32.const 4))
    local.set $loc_arg_var
    (i32.sub (local.get $args_rest) (i32.const 1))
    local.tee $args_rest
    br_if $set_arg
)
end
local.get $pap
return
)
(func $__exec_pap (param $pap i32) (result i32)
(local $p_0 i32)
(local $p_1 i32)
(block $__case0
(block $__case1
(block $__case2
(block $__case3
(block $__case4
(block $__case5
(block $__case6
(block $__case7
(block $__case8
(block $__case9
(block $__case10
(block $__case11
(block $__case12
(block $__case13
(block $__case14
(block $__case15
(block $__case16
(block $__case17
(block $__case18
(block $__case19
(block $__case20
(block $__case21
(block $__case22
(block $__case23
(block $__case24
(block $__case25
(block $__case26
(block $__case27
(block $__case28
(block $__case29
local.get $pap
i32.const 8
i32.add
i32.load
br_table 
$__case28 $__case27 $__case26 $__case25 $__case24 $__case23 $__case22 $__case21 $__case20 $__case19 $__case18 $__case17 $__case16 $__case15 $__case14 $__case13 $__case12 $__case11 $__case10 $__case9 $__case8 $__case7 $__case6 $__case5 $__case4 $__case3 $__case2 $__case1 $__case0 )
)
local.get $pap
i32.const 16
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $pap
i32.const 20
i32.add
i32.load
local.tee $p_1
local.get $p_1
local.get $p_1
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $p_0
i32.const 8
i32.add
i32.load
local.get $p_1
i32.const 8
i32.add
i32.load
i32.add
call $__make_num
local.get $p_0
call $__dec
local.get $p_1
call $__dec
local.get $pap
call $__dec
return
)
local.get $pap
i32.const 16
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $pap
i32.const 20
i32.add
i32.load
local.tee $p_1
local.get $p_1
local.get $p_1
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $p_0
i32.const 8
i32.add
i32.load
local.get $p_1
i32.const 8
i32.add
i32.load
i32.sub
call $__make_num
local.get $p_0
call $__dec
local.get $p_1
call $__dec
local.get $pap
call $__dec
return
)
local.get $pap
i32.const 16
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $pap
i32.const 20
i32.add
i32.load
local.tee $p_1
local.get $p_1
local.get $p_1
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $p_0
i32.const 8
i32.add
i32.load
local.get $p_1
i32.const 8
i32.add
i32.load
i32.mul
call $__make_num
local.get $p_0
call $__dec
local.get $p_1
call $__dec
local.get $pap
call $__dec
return
)
local.get $pap
i32.const 16
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $pap
i32.const 20
i32.add
i32.load
local.tee $p_1
local.get $p_1
local.get $p_1
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $p_0
i32.const 8
i32.add
i32.load
local.get $p_1
i32.const 8
i32.add
i32.load
i32.div_s
call $__make_num
local.get $p_0
call $__dec
local.get $p_1
call $__dec
local.get $pap
call $__dec
return
)
local.get $pap
i32.const 16
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $pap
i32.const 20
i32.add
i32.load
local.tee $p_1
local.get $p_1
local.get $p_1
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $p_0
i32.const 8
i32.add
i32.load
local.get $p_1
i32.const 8
i32.add
i32.load
i32.rem_s
call $__make_num
local.get $p_0
call $__dec
local.get $p_1
call $__dec
local.get $pap
call $__dec
return
)
local.get $pap
i32.const 16
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $pap
i32.const 20
i32.add
i32.load
local.tee $p_1
local.get $p_1
local.get $p_1
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $p_0
i32.load
local.get $p_1
i32.load
i32.and
call $__make_no_arg
local.get $p_0
call $__dec
local.get $p_1
call $__dec
local.get $pap
call $__dec
return
)
local.get $pap
i32.const 16
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $pap
i32.const 20
i32.add
i32.load
local.tee $p_1
local.get $p_1
local.get $p_1
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $p_0
i32.load
local.get $p_1
i32.load
i32.or
call $__make_no_arg
local.get $p_0
call $__dec
local.get $p_1
call $__dec
local.get $pap
call $__dec
return
)
local.get $pap
i32.const 16
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $p_0
i32.load
i32.eqz
call $__make_no_arg
local.get $p_0
call $__dec
local.get $pap
call $__dec
return
)
local.get $pap
i32.const 16
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $pap
i32.const 20
i32.add
i32.load
local.tee $p_1
local.get $p_1
local.get $p_1
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $p_0
i32.const 8
i32.add
i32.load
local.get $p_1
i32.const 8
i32.add
i32.load
i32.eq
call $__make_no_arg
local.get $p_0
call $__dec
local.get $p_1
call $__dec
local.get $pap
call $__dec
return
)
local.get $pap
i32.const 16
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $pap
i32.const 20
i32.add
i32.load
local.tee $p_1
local.get $p_1
local.get $p_1
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $p_0
i32.const 8
i32.add
i32.load
local.get $p_1
i32.const 8
i32.add
i32.load
i32.gt_s
call $__make_no_arg
local.get $p_0
call $__dec
local.get $p_1
call $__dec
local.get $pap
call $__dec
return
)
local.get $pap
i32.const 16
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $pap
i32.const 20
i32.add
i32.load
local.tee $p_1
local.get $p_1
local.get $p_1
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $p_0
i32.const 8
i32.add
i32.load
local.get $p_1
i32.const 8
i32.add
i32.load
i32.lt_s
call $__make_no_arg
local.get $p_0
call $__dec
local.get $p_1
call $__dec
local.get $pap
call $__dec
return
)
local.get $pap
i32.const 16
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $pap
i32.const 20
i32.add
i32.load
local.tee $p_1
local.get $p_1
local.get $p_1
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $p_0
i32.const 8
i32.add
i32.load
local.get $p_1
i32.const 8
i32.add
i32.load
i32.ge_s
call $__make_no_arg
local.get $p_0
call $__dec
local.get $p_1
call $__dec
local.get $pap
call $__dec
return
)
local.get $pap
i32.const 16
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $pap
i32.const 20
i32.add
i32.load
local.tee $p_1
local.get $p_1
local.get $p_1
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $p_0
i32.const 8
i32.add
i32.load
local.get $p_1
i32.const 8
i32.add
i32.load
i32.le_s
call $__make_no_arg
local.get $p_0
call $__dec
local.get $p_1
call $__dec
local.get $pap
call $__dec
return
)
local.get $pap
i32.const 16
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $pap
i32.const 20
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $pap
i32.const 24
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
call $fun_arbre
local.get $pap
call $__dec
return
)
local.get $pap
i32.const 16
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
call $fun_est_feuille
local.get $pap
call $__dec
return
)
local.get $pap
i32.const 16
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
call $fun_valeur
local.get $pap
call $__dec
return
)
local.get $pap
i32.const 16
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
call $fun_fils
local.get $pap
call $__dec
return
)
local.get $pap
i32.const 16
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
call $fun_fils_gauche
local.get $pap
call $__dec
return
)
local.get $pap
i32.const 16
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
call $fun_fils_droit
local.get $pap
call $__dec
return
)
local.get $pap
i32.const 16
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $pap
i32.const 20
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
call $fun_fmap_arbre
local.get $pap
call $__dec
return
)
local.get $pap
i32.const 16
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $pap
i32.const 20
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
call $fun_max
local.get $pap
call $__dec
return
)
local.get $pap
i32.const 16
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $pap
i32.const 20
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
call $fun_min
local.get $pap
call $__dec
return
)
local.get $pap
i32.const 16
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
call $fun_etage_plus_haute_feuille
local.get $pap
call $__dec
return
)
local.get $pap
i32.const 16
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
call $fun_hauteur
local.get $pap
call $__dec
return
)
local.get $pap
i32.const 16
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $pap
i32.const 20
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
local.get $pap
i32.const 24
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
call $fun_fold_arbre
local.get $pap
call $__dec
return
)
local.get $pap
i32.const 16
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
call $fun_max_a
local.get $pap
call $__dec
return
)
local.get $pap
i32.const 16
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
call $fun_min_a
local.get $pap
call $__dec
return
)
local.get $pap
i32.const 16
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
call $fun_somme
local.get $pap
call $__dec
return
)
local.get $pap
i32.const 16
i32.add
i32.load
local.tee $p_0
local.get $p_0
local.get $p_0
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref
call $fun_nb_noeuds
local.get $pap
call $__dec
return
)
(func $__dec (param $var i32)
 (local $args_left i32)
 (local $arg i32)
local.get $var
 if
local.get $var
local.get $var
i32.const 4
i32.add
i32.load
i32.const -1
i32.add
call $__set_ref
local.get $var
i32.const 4
i32.add
  i32.load
  i32.eqz
  if
local.get $var
i32.load
    i32.const 5
    i32.eq
    if
local.get $var
i32.const 12
i32.add
      i32.load
      local.set $args_left
local.get $var
i32.const 16
i32.add
      local.set $arg
      (block $dec_end
        (loop $dec_loop
          local.get $args_left
          i32.eqz
          br_if $dec_end
          local.get $arg
          i32.load
          call $__dec
          (i32.add (local.get $arg) (i32.const 4))
          local.set $arg
          (i32.sub (local.get $args_left) (i32.const 1))
          local.set $args_left
          br $dec_loop
        )
      )
    end
    local.get $var
    i32.load
    i32.const 3
    i32.eq
    if ;; si de type LIST
local.get $var
i32.const 8
i32.add
      i32.load   ;; @arg 1
      call $__dec;; dec arg 1
local.get $var
i32.const 12
i32.add
      i32.load   ;; @arg 2
      call $__dec;; dec arg 2
    end
  end
 end
)
(func $fun_arbre (export "arbre")(param $var_v i32) (param $var_g i32) (param $var_d i32) (result i32)
(local $var_fs i32)
(local $__intern_var i32)
(local $var_a i32)

;;inc
local.get $var_g
local.get $var_g
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;inc
local.get $var_d
local.get $var_d
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;ctor
local.get $var_g
local.get $var_d
call $__make_list
local.set $var_fs

;;inc
local.get $var_v
local.get $var_v
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;ctor
local.get $var_v
local.get $var_fs
call $__make_list
return
)
(func $fun_feuille (export "feuille")(result i32)
(local $var_r i32)
(local $__intern_var i32)

;;let

;;ctor
i32.const 2
call $__make_no_arg
return
)
(func $fun_est_feuille (export "est_feuille")(param $var_a i32) (result i32)
(local $w1 i32)
(local $__intern_var i32)
(local $w3 i32)
(local $w2 i32)
(local $var_r i32)

;;case
(block $__case0
(block $__case1
(block $__case2
(block $__case3
(block $__case4
local.get $var_a
i32.load
(br_table 
$__case4 $__case3 $__case2 $__case1 $__case0 )
)

;;let

;;reset
local.get $var_a
call $__reset
local.set $w1

;;let

;;reuse
local.get $w1
local.get $w1
i32.load
i32.const 1
i32.eq
local.get $w1
i32.load
i32.const 3
i32.le_s
i32.or
i32.and
if
local.get $w1
i32.const 1
i32.store
local.get $w1
i32.const 4
i32.add
i32.const 1
i32.store
local.get $w1
local.set $__intern_var
else
i32.const 1
call $__make_no_arg
local.set $__intern_var
end
local.get $__intern_var
return
)

;;let

;;reset
local.get $var_a
call $__reset
local.set $w2

;;let

;;reuse
local.get $w2
local.get $w2
i32.load
i32.const 1
i32.eq
local.get $w2
i32.load
i32.const 3
i32.le_s
i32.or
i32.and
if
local.get $w2
i32.const 1
i32.store
local.get $w2
i32.const 4
i32.add
i32.const 1
i32.store
local.get $w2
local.set $__intern_var
else
i32.const 1
call $__make_no_arg
local.set $__intern_var
end
local.get $__intern_var
return
)

;;let

;;reset
local.get $var_a
call $__reset
local.set $w3

;;let

;;reuse
local.get $w3
local.get $w3
i32.load
i32.const 1
i32.eq
local.get $w3
i32.load
i32.const 3
i32.le_s
i32.or
i32.and
if
local.get $w3
i32.const 1
i32.store
local.get $w3
i32.const 4
i32.add
i32.const 1
i32.store
local.get $w3
local.set $__intern_var
else
i32.const 1
call $__make_no_arg
local.set $__intern_var
end
local.get $__intern_var
return
)

;;dec
local.get $var_a
call $__dec

;;let

;;ctor
i32.const 0
call $__make_no_arg
return
)

;;dec
local.get $var_a
call $__dec

;;let

;;ctor
i32.const 1
call $__make_no_arg
return
)
(func $fun_valeur (export "valeur")(param $var_a i32) (result i32)
(local $var_r i32)
(local $__intern_var i32)

;;let

;;proj
local.get $var_a
i32.const 8
i32.add
i32.load
local.set $var_r

;;inc
local.get $var_r
local.get $var_r
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;ret
local.get $var_r
return
)
(func $fun_fils (export "fils")(param $var_a i32) (result i32)
(local $var_r i32)
(local $__intern_var i32)

;;let

;;proj
local.get $var_a
i32.const 12
i32.add
i32.load
local.set $var_r

;;inc
local.get $var_r
local.get $var_r
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;ret
local.get $var_r
return
)
(func $fun_fils_gauche (export "fils_gauche")(param $var_a i32) (result i32)
(local $var_r i32)
(local $var_fs i32)
(local $__intern_var i32)

;;let

;;fncall
local.get $var_a
call $fun_fils
local.set $var_fs

;;let

;;proj
local.get $var_fs
i32.const 8
i32.add
i32.load
local.set $var_r

;;inc
local.get $var_r
local.get $var_r
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;dec
local.get $var_fs
call $__dec

;;ret
local.get $var_r
return
)
(func $fun_fils_droit (export "fils_droit")(param $var_a i32) (result i32)
(local $var_fs i32)
(local $__intern_var i32)
(local $var_r i32)

;;let

;;fncall
local.get $var_a
call $fun_fils
local.set $var_fs

;;let

;;proj
local.get $var_fs
i32.const 12
i32.add
i32.load
local.set $var_r

;;inc
local.get $var_r
local.get $var_r
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;dec
local.get $var_fs
call $__dec

;;ret
local.get $var_r
return
)
(func $fun_fmap_arbre (export "fmap_arbre")(param $var_fun i32) (param $var_a i32) (result i32)
(local $var_d i32)
(local $var_na i32)
(local $var_nd i32)
(local $var_ng i32)
(local $var_g i32)
(local $__intern_var i32)
(local $var_nv i32)
(local $var_v i32)
(local $var_est_f i32)

;;inc
local.get $var_a
local.get $var_a
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;fncall
local.get $var_a
call $fun_est_feuille
local.set $var_est_f

;;case
(block $__case0
(block $__case1
local.get $var_est_f
i32.load
(br_table 
$__case1 $__case0 )
)

;;dec
local.get $var_est_f
call $__dec

;;let

;;fncall
local.get $var_a
call $fun_valeur
local.set $var_v

;;inc
local.get $var_fun
local.get $var_fun
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let
local.get $var_fun
call $__copy_pap
local.set $__intern_var
local.get $var_fun
call $__dec
local.get $__intern_var
i32.const 12
i32.add
local.get $__intern_var
i32.const 12
i32.add
i32.load
i32.const 1
i32.add
i32.store
local.get $__intern_var
i32.const 12
i32.add
i32.load
i32.const 4
i32.mul
local.get $__intern_var
i32.const 12
i32.add
i32.add
local.get $var_v
i32.store
local.get $__intern_var
i32.const 12
i32.add
i32.load
local.get $__intern_var
i32.const 8
i32.add
i32.load
call $__nb_args
i32.eq
if
    local.get $__intern_var
    call $__exec_pap
    local.set $__intern_var
end
local.get $__intern_var
local.set $var_nv

;;let

;;fncall
local.get $var_a
call $fun_fils_gauche
local.set $var_g

;;inc
local.get $var_fun
local.get $var_fun
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;fncall
local.get $var_fun
local.get $var_g
call $fun_fmap_arbre
local.set $var_ng

;;let

;;fncall
local.get $var_a
call $fun_fils_droit
local.set $var_d

;;dec
local.get $var_a
call $__dec

;;let

;;fncall
local.get $var_fun
local.get $var_d
call $fun_fmap_arbre
local.set $var_nd

;;let

;;fncall
local.get $var_nv
local.get $var_ng
local.get $var_nd
call $fun_arbre
local.set $var_na

;;dec
local.get $var_nd
call $__dec

;;dec
local.get $var_ng
call $__dec

;;dec
local.get $var_nv
call $__dec

;;ret
local.get $var_na
return
)

;;dec
local.get $var_est_f
call $__dec

;;dec
local.get $var_fun
call $__dec

;;ret
local.get $var_a
return
)
(func $fun_max (export "max")(param $var_a i32) (param $var_b i32) (result i32)
(local $var_t i32)
(local $__intern_var i32)

;;inc
local.get $var_a
local.get $var_a
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;inc
local.get $var_b
local.get $var_b
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;fncall
local.get $var_a
i32.const 8
i32.add
i32.load
local.get $var_b
i32.const 8
i32.add
i32.load
i32.ge_s
call $__make_no_arg
local.get $var_a
call $__dec
local.get $var_b
call $__dec
local.set $var_t

;;case
(block $__case0
(block $__case1
local.get $var_t
i32.load
(br_table 
$__case1 $__case0 )
)

;;dec
local.get $var_t
call $__dec

;;dec
local.get $var_a
call $__dec

;;ret
local.get $var_b
return
)

;;dec
local.get $var_t
call $__dec

;;dec
local.get $var_b
call $__dec

;;ret
local.get $var_a
return
)
(func $fun_min (export "min")(param $var_a i32) (param $var_b i32) (result i32)
(local $var_t i32)
(local $__intern_var i32)

;;inc
local.get $var_a
local.get $var_a
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;inc
local.get $var_b
local.get $var_b
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;fncall
local.get $var_a
i32.const 8
i32.add
i32.load
local.get $var_b
i32.const 8
i32.add
i32.load
i32.le_s
call $__make_no_arg
local.get $var_a
call $__dec
local.get $var_b
call $__dec
local.set $var_t

;;case
(block $__case0
(block $__case1
local.get $var_t
i32.load
(br_table 
$__case1 $__case0 )
)

;;dec
local.get $var_t
call $__dec

;;dec
local.get $var_a
call $__dec

;;ret
local.get $var_b
return
)

;;dec
local.get $var_t
call $__dec

;;dec
local.get $var_b
call $__dec

;;ret
local.get $var_a
return
)
(func $fun_etage_plus_haute_feuille (export "etage_plus_haute_feuille")(param $var_a i32) (result i32)
(local $var_hfs i32)
(local $var_g i32)
(local $var_r i32)
(local $var_est_f i32)
(local $var_h i32)
(local $var_d i32)
(local $var_hg i32)
(local $var_hd i32)
(local $__intern_var i32)
(local $var_n1 i32)

;;inc
local.get $var_a
local.get $var_a
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;fncall
local.get $var_a
call $fun_est_feuille
local.set $var_est_f

;;case
(block $__case0
(block $__case1
local.get $var_est_f
i32.load
(br_table 
$__case1 $__case0 )
)

;;dec
local.get $var_est_f
call $__dec

;;let

;;fncall
local.get $var_a
call $fun_fils_gauche
local.set $var_g

;;let

;;fncall
local.get $var_g
call $fun_etage_plus_haute_feuille
local.set $var_hg

;;let

;;fncall
local.get $var_a
call $fun_fils_droit
local.set $var_d

;;dec
local.get $var_a
call $__dec

;;let

;;fncall
local.get $var_d
call $fun_etage_plus_haute_feuille
local.set $var_hd

;;let

;;fncall
local.get $var_hg
local.get $var_hd
call $fun_min
local.set $var_hfs

;;let

;;num
i32.const 1
call $__make_num
local.set $var_n1

;;let

;;fncall
local.get $var_hfs
i32.const 8
i32.add
i32.load
local.get $var_n1
i32.const 8
i32.add
i32.load
i32.add
call $__make_num
local.get $var_hfs
call $__dec
local.get $var_n1
call $__dec
return
)

;;dec
local.get $var_est_f
call $__dec

;;dec
local.get $var_a
call $__dec

;;let

;;num
i32.const 1
call $__make_num
return
)
(func $fun_hauteur (export "hauteur")(param $var_a i32) (result i32)
(local $var_n1 i32)
(local $var_g i32)
(local $var_d i32)
(local $var_est_f i32)
(local $var_r i32)
(local $__intern_var i32)
(local $var_hd i32)
(local $var_h i32)
(local $var_hfs i32)
(local $var_hg i32)

;;inc
local.get $var_a
local.get $var_a
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;fncall
local.get $var_a
call $fun_est_feuille
local.set $var_est_f

;;case
(block $__case0
(block $__case1
local.get $var_est_f
i32.load
(br_table 
$__case1 $__case0 )
)

;;dec
local.get $var_est_f
call $__dec

;;let

;;fncall
local.get $var_a
call $fun_fils_gauche
local.set $var_g

;;let

;;fncall
local.get $var_g
call $fun_hauteur
local.set $var_hg

;;let

;;fncall
local.get $var_a
call $fun_fils_droit
local.set $var_d

;;dec
local.get $var_a
call $__dec

;;let

;;fncall
local.get $var_d
call $fun_hauteur
local.set $var_hd

;;let

;;fncall
local.get $var_hg
local.get $var_hd
call $fun_max
local.set $var_hfs

;;let

;;num
i32.const 1
call $__make_num
local.set $var_n1

;;let

;;fncall
local.get $var_hfs
i32.const 8
i32.add
i32.load
local.get $var_n1
i32.const 8
i32.add
i32.load
i32.add
call $__make_num
local.get $var_hfs
call $__dec
local.get $var_n1
call $__dec
return
)

;;dec
local.get $var_est_f
call $__dec

;;dec
local.get $var_a
call $__dec

;;let

;;num
i32.const 0
call $__make_num
return
)
(func $fun_fold_arbre (export "fold_arbre")(param $var_fun i32) (param $var_a i32) (param $var_acc i32) (result i32)
(local $var_v i32)
(local $__intern_var i32)
(local $var_vd i32)
(local $var_est_f i32)
(local $var_nv i32)
(local $var_g i32)
(local $var_vg i32)
(local $var_d i32)
(local $var_nfv i32)

;;inc
local.get $var_a
local.get $var_a
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;fncall
local.get $var_a
call $fun_est_feuille
local.set $var_est_f

;;case
(block $__case0
(block $__case1
local.get $var_est_f
i32.load
(br_table 
$__case1 $__case0 )
)

;;dec
local.get $var_est_f
call $__dec

;;let

;;fncall
local.get $var_a
call $fun_fils_gauche
local.set $var_g

;;inc
local.get $var_fun
local.get $var_fun
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;fncall
local.get $var_fun
local.get $var_g
local.get $var_acc
call $fun_fold_arbre
local.set $var_vg

;;let

;;fncall
local.get $var_a
call $fun_fils_droit
local.set $var_d

;;inc
local.get $var_fun
local.get $var_fun
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;fncall
local.get $var_fun
local.get $var_d
local.get $var_vg
call $fun_fold_arbre
local.set $var_vd

;;dec
local.get $var_vg
call $__dec

;;let

;;fncall
local.get $var_a
call $fun_valeur
local.set $var_v

;;dec
local.get $var_a
call $__dec

;;let
local.get $var_fun
call $__copy_pap
local.set $__intern_var
local.get $var_fun
call $__dec
local.get $__intern_var
i32.const 12
i32.add
local.get $__intern_var
i32.const 12
i32.add
i32.load
i32.const 1
i32.add
i32.store
local.get $__intern_var
i32.const 12
i32.add
i32.load
i32.const 4
i32.mul
local.get $__intern_var
i32.const 12
i32.add
i32.add
local.get $var_vd
i32.store
local.get $__intern_var
i32.const 12
i32.add
i32.load
local.get $__intern_var
i32.const 8
i32.add
i32.load
call $__nb_args
i32.eq
if
    local.get $__intern_var
    call $__exec_pap
    local.set $__intern_var
end
local.get $__intern_var
local.set $var_nfv

;;let
local.get $var_nfv
call $__copy_pap
local.set $__intern_var
local.get $var_nfv
call $__dec
local.get $__intern_var
i32.const 12
i32.add
local.get $__intern_var
i32.const 12
i32.add
i32.load
i32.const 1
i32.add
i32.store
local.get $__intern_var
i32.const 12
i32.add
i32.load
i32.const 4
i32.mul
local.get $__intern_var
i32.const 12
i32.add
i32.add
local.get $var_v
i32.store
local.get $__intern_var
i32.const 12
i32.add
i32.load
local.get $__intern_var
i32.const 8
i32.add
i32.load
call $__nb_args
i32.eq
if
    local.get $__intern_var
    call $__exec_pap
    local.set $__intern_var
end
local.get $__intern_var
return
)

;;dec
local.get $var_est_f
call $__dec

;;dec
local.get $var_fun
call $__dec

;;dec
local.get $var_a
call $__dec

;;inc
local.get $var_acc
local.get $var_acc
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;ret
local.get $var_acc
return
)
(func $fun_arbre_test (export "arbre_test")(result i32)
(local $var_a3 i32)
(local $__intern_var i32)
(local $var_v2 i32)
(local $var_v4 i32)
(local $var_a321 i32)
(local $var_v1 i32)
(local $var_v3 i32)
(local $var_a3224 i32)
(local $var_a322 i32)
(local $var_nil i32)
(local $var_a32 i32)

;;let

;;num
i32.const 1
call $__make_num
local.set $var_v1

;;let

;;num
i32.const 2
call $__make_num
local.set $var_v2

;;let

;;num
i32.const 3
call $__make_num
local.set $var_v3

;;let

;;num
i32.const 4
call $__make_num
local.set $var_v4

;;let

;;fncall
call $fun_feuille
local.set $var_nil

;;let

;;fncall
local.get $var_v4
local.get $var_nil
local.get $var_nil
call $fun_arbre
local.set $var_a3224

;;dec
local.get $var_v4
call $__dec

;;let

;;fncall
local.get $var_v2
local.get $var_nil
local.get $var_a3224
call $fun_arbre
local.set $var_a322

;;dec
local.get $var_a3224
call $__dec

;;let

;;fncall
local.get $var_v1
local.get $var_nil
local.get $var_nil
call $fun_arbre
local.set $var_a321

;;dec
local.get $var_nil
call $__dec

;;dec
local.get $var_v1
call $__dec

;;let

;;fncall
local.get $var_v2
local.get $var_a321
local.get $var_a322
call $fun_arbre
local.set $var_a32

;;dec
local.get $var_a322
call $__dec

;;dec
local.get $var_v2
call $__dec

;;let

;;fncall
local.get $var_v3
local.get $var_a32
local.get $var_a321
call $fun_arbre
local.set $var_a3

;;dec
local.get $var_a321
call $__dec

;;dec
local.get $var_a32
call $__dec

;;dec
local.get $var_v3
call $__dec

;;ret
local.get $var_a3
return
)
(func $fun_add_arbre (export "add_arbre")(result i32)
(local $var_n i32)
(local $var_a i32)
(local $__intern_var i32)
(local $var_r i32)
(local $var_f i32)

;;let

;;num
i32.const 5
call $__make_num
local.set $var_n

;;let

;;pap
i32.const 0
call $__make_pap
local.set $__intern_var
local.get $__intern_var
i32.const 16
i32.add
local.get $var_n
i32.store
local.get $__intern_var
i32.const 12
i32.add
i32.const 1
i32.store
local.get $__intern_var
local.set $var_f

;;let

;;fncall
call $fun_arbre_test
local.set $var_a

;;let

;;fncall
local.get $var_f
local.get $var_a
call $fun_fmap_arbre
return
)
(func $fun_max_a (export "max_a")(param $var_a i32) (result i32)
(local $var_r i32)
(local $__intern_var i32)
(local $var_vg i32)
(local $var_vd i32)
(local $var_temp i32)
(local $var_d i32)
(local $var_n i32)
(local $var_g i32)
(local $var_v i32)
(local $var_t i32)

;;inc
local.get $var_a
local.get $var_a
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;fncall
local.get $var_a
call $fun_est_feuille
local.set $var_t

;;case
(block $__case0
(block $__case1
local.get $var_t
i32.load
(br_table 
$__case1 $__case0 )
)

;;dec
local.get $var_t
call $__dec

;;let

;;fncall
local.get $var_a
call $fun_fils_gauche
local.set $var_g

;;let

;;fncall
local.get $var_g
call $fun_max_a
local.set $var_vg

;;let

;;fncall
local.get $var_a
call $fun_fils_droit
local.set $var_d

;;let

;;fncall
local.get $var_d
call $fun_max_a
local.set $var_vd

;;let

;;fncall
local.get $var_vd
local.get $var_vg
call $fun_max
local.set $var_temp

;;let

;;fncall
local.get $var_a
call $fun_valeur
local.set $var_v

;;dec
local.get $var_a
call $__dec

;;let

;;fncall
local.get $var_temp
local.get $var_v
call $fun_max
return
)

;;dec
local.get $var_t
call $__dec

;;dec
local.get $var_a
call $__dec

;;let

;;num
i32.const 0
call $__make_num
return
)
(func $fun_max_arbre (export "max_arbre")(result i32)
(local $var_r i32)
(local $__intern_var i32)
(local $var_a i32)

;;let

;;fncall
call $fun_arbre_test
local.set $var_a

;;let

;;fncall
local.get $var_a
call $fun_max_a
return
)
(func $fun_min_a (export "min_a")(param $var_a i32) (result i32)
(local $var_vg i32)
(local $__intern_var i32)
(local $var_vd i32)
(local $var_v i32)
(local $var_n i32)
(local $var_r i32)
(local $var_g i32)
(local $var_t i32)
(local $var_temp i32)
(local $var_d i32)

;;inc
local.get $var_a
local.get $var_a
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;fncall
local.get $var_a
call $fun_est_feuille
local.set $var_t

;;case
(block $__case0
(block $__case1
local.get $var_t
i32.load
(br_table 
$__case1 $__case0 )
)

;;dec
local.get $var_t
call $__dec

;;let

;;fncall
local.get $var_a
call $fun_fils_gauche
local.set $var_g

;;let

;;fncall
local.get $var_g
call $fun_min_a
local.set $var_vg

;;let

;;fncall
local.get $var_a
call $fun_fils_droit
local.set $var_d

;;let

;;fncall
local.get $var_d
call $fun_min_a
local.set $var_vd

;;let

;;fncall
local.get $var_vd
local.get $var_vg
call $fun_min
local.set $var_temp

;;let

;;fncall
local.get $var_a
call $fun_valeur
local.set $var_v

;;dec
local.get $var_a
call $__dec

;;let

;;fncall
local.get $var_temp
local.get $var_v
call $fun_min
return
)

;;dec
local.get $var_t
call $__dec

;;dec
local.get $var_a
call $__dec

;;let

;;num
i32.const 10000
call $__make_num
return
)
(func $fun_min_arbre (export "min_arbre")(result i32)
(local $var_r i32)
(local $var_f i32)
(local $__intern_var i32)
(local $var_n i32)
(local $var_a i32)

;;let

;;fncall
call $fun_arbre_test
local.set $var_a

;;let

;;pap
i32.const 21
call $__make_pap
local.set $__intern_var
local.get $__intern_var
local.set $var_f

;;let

;;num
i32.const 110
call $__make_num
local.set $var_n

;;let

;;fncall
local.get $var_f
local.get $var_a
local.get $var_n
call $fun_fold_arbre
local.set $var_r

;;dec
local.get $var_n
call $__dec

;;ret
local.get $var_r
return
)
(func $fun_hauteur_test (export "hauteur_test")(result i32)
(local $__intern_var i32)
(local $var_a i32)
(local $var_r i32)

;;let

;;fncall
call $fun_arbre_test
local.set $var_a

;;let

;;fncall
local.get $var_a
call $fun_hauteur
return
)
(func $fun_ephf (export "ephf")(result i32)
(local $var_a i32)
(local $var_r i32)
(local $__intern_var i32)

;;let

;;fncall
call $fun_arbre_test
local.set $var_a

;;let

;;fncall
local.get $var_a
call $fun_etage_plus_haute_feuille
return
)
(func $fun_somme (export "somme")(param $var_a i32) (result i32)
(local $var_r i32)
(local $__intern_var i32)
(local $var_n i32)
(local $var_f i32)

;;let

;;pap
i32.const 0
call $__make_pap
local.set $__intern_var
local.get $__intern_var
local.set $var_f

;;let

;;num
i32.const 0
call $__make_num
local.set $var_n

;;inc
local.get $var_a
local.get $var_a
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;fncall
local.get $var_f
local.get $var_a
local.get $var_n
call $fun_fold_arbre
local.set $var_r

;;dec
local.get $var_n
call $__dec

;;ret
local.get $var_r
return
)
(func $fun_somme_test (export "somme_test")(result i32)
(local $var_a i32)
(local $__intern_var i32)
(local $var_r i32)

;;let

;;fncall
call $fun_arbre_test
local.set $var_a

;;let

;;fncall
local.get $var_a
call $fun_somme
local.set $var_r

;;dec
local.get $var_a
call $__dec

;;ret
local.get $var_r
return
)
(func $fun_nb_noeuds (export "nb_noeuds")(param $var_a i32) (result i32)
(local $var_vd i32)
(local $var_g i32)
(local $__intern_var i32)
(local $var_v i32)
(local $var_d i32)
(local $var_est_f i32)
(local $var_temp i32)
(local $var_r i32)
(local $var_vg i32)

;;inc
local.get $var_a
local.get $var_a
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;fncall
local.get $var_a
call $fun_est_feuille
local.set $var_est_f

;;case
(block $__case0
(block $__case1
local.get $var_est_f
i32.load
(br_table 
$__case1 $__case0 )
)

;;dec
local.get $var_est_f
call $__dec

;;let

;;fncall
local.get $var_a
call $fun_fils_gauche
local.set $var_g

;;let

;;fncall
local.get $var_g
call $fun_nb_noeuds
local.set $var_vg

;;let

;;fncall
local.get $var_a
call $fun_fils_droit
local.set $var_d

;;dec
local.get $var_a
call $__dec

;;let

;;fncall
local.get $var_d
call $fun_nb_noeuds
local.set $var_vd

;;let

;;fncall
local.get $var_vd
i32.const 8
i32.add
i32.load
local.get $var_vg
i32.const 8
i32.add
i32.load
i32.add
call $__make_num
local.get $var_vd
call $__dec
local.get $var_vg
call $__dec
local.set $var_temp

;;let

;;num
i32.const 1
call $__make_num
local.set $var_v

;;let

;;fncall
local.get $var_temp
i32.const 8
i32.add
i32.load
local.get $var_v
i32.const 8
i32.add
i32.load
i32.add
call $__make_num
local.get $var_temp
call $__dec
local.get $var_v
call $__dec
return
)

;;dec
local.get $var_est_f
call $__dec

;;dec
local.get $var_a
call $__dec

;;let

;;num
i32.const 0
call $__make_num
return
)
(func $fun_nb_noeuds_test (export "nb_noeuds_test")(result i32)
(local $var_r i32)
(local $__intern_var i32)
(local $var_a i32)

;;let

;;fncall
call $fun_arbre_test
local.set $var_a

;;let

;;fncall
local.get $var_a
call $fun_nb_noeuds
return
)
)
