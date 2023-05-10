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
    ;; mise à jour de memory[0]
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
    (local $__intern_var i32)
local.get $var
local.get $var
i32.const 4
i32.add
i32.load
i32.const 1
i32.sub
call $__set_ref
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
(func $__nb_args (param $id i32) (result i32)
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
    local.get $id
  (br_table 
$__case13 $__case12 $__case11 $__case10 $__case9 $__case8 $__case7 $__case6 $__case5 $__case4 $__case3 $__case2 $__case1 $__case0 )
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
  local.set $args_rest
  local.get $args_rest
  br_if $set_arg
)
local.get $pap
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
local.get $pap
i32.const 8
i32.add
i32.load
br_table 
$__case13 $__case12 $__case11 $__case10 $__case9 $__case8 $__case7 $__case6 $__case5 $__case4 $__case3 $__case2 $__case1 $__case0 )
)
(i32.add (local.get $pap) (i32.const 16))
i32.load
(i32.add (local.get $pap) (i32.const 20))
i32.load
call $fun_papcall
return
)
(i32.add (local.get $pap) (i32.const 16))
i32.load
local.set $p_0
(i32.add (local.get $pap) (i32.const 20))
i32.load
local.set $p_1
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
return
)
(i32.add (local.get $pap) (i32.const 16))
i32.load
local.set $p_0
(i32.add (local.get $pap) (i32.const 20))
i32.load
local.set $p_1
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
return
)
(i32.add (local.get $pap) (i32.const 16))
i32.load
local.set $p_0
(i32.add (local.get $pap) (i32.const 20))
i32.load
local.set $p_1
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
return
)
(i32.add (local.get $pap) (i32.const 16))
i32.load
local.set $p_0
(i32.add (local.get $pap) (i32.const 20))
i32.load
local.set $p_1
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
return
)
(i32.add (local.get $pap) (i32.const 16))
i32.load
local.set $p_0
(i32.add (local.get $pap) (i32.const 20))
i32.load
local.set $p_1
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
return
)
(i32.add (local.get $pap) (i32.const 16))
i32.load
local.set $p_0
(i32.add (local.get $pap) (i32.const 20))
i32.load
local.set $p_1
local.get $p_0
i32.load
local.get $p_1
i32.load
i32.and
call $__make_no_arg
return
)
(i32.add (local.get $pap) (i32.const 16))
i32.load
local.set $p_0
(i32.add (local.get $pap) (i32.const 20))
i32.load
local.set $p_1
local.get $p_0
i32.load
local.get $p_1
i32.load
i32.or
call $__make_no_arg
return
)
(i32.add (local.get $pap) (i32.const 16))
i32.load
local.set $p_0
local.get $p_0
i32.load
i32.eqz
call $__make_no_arg
return
)
(i32.add (local.get $pap) (i32.const 16))
i32.load
local.set $p_0
(i32.add (local.get $pap) (i32.const 20))
i32.load
local.set $p_1
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
return
)
(i32.add (local.get $pap) (i32.const 16))
i32.load
local.set $p_0
(i32.add (local.get $pap) (i32.const 20))
i32.load
local.set $p_1
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
return
)
(i32.add (local.get $pap) (i32.const 16))
i32.load
local.set $p_0
(i32.add (local.get $pap) (i32.const 20))
i32.load
local.set $p_1
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
return
)
(i32.add (local.get $pap) (i32.const 16))
i32.load
local.set $p_0
(i32.add (local.get $pap) (i32.const 20))
i32.load
local.set $p_1
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
return
)
(i32.add (local.get $pap) (i32.const 16))
i32.load
local.set $p_0
(i32.add (local.get $pap) (i32.const 20))
i32.load
local.set $p_1
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
return
)
(func $fun_papcall (export "papcall")(param $n i32) (param $m i32) (result i32)
(local $pa i32)
(local $__intern_var i32)

;;let

;;pap
i32.const 2
call $__make_pap
local.set $__intern_var
local.get $__intern_var
local.set $pa

;;inc
local.get $pa
local.get $pa
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;papcall
local.get $pa
call $__copy_pap
local.get $pa
i32.const 16
i32.add
local.get $pa
i32.const 12
i32.add
i32.load
i32.const 4
i32.mul
local.get $n
i32.store
local.get $pa
i32.const 12
i32.add
local.get $pa
i32.const 12
i32.add
i32.load
i32.const 1
i32.add
i32.store
local.get $pa
i32.const 12
i32.add
i32.load
local.get $pa
i32.const 8
i32.add
i32.load
call $__nb_args
i32.eq
if
local.get $pa
call $__exec_pap
local.set $__intern_var
else
local.get $pa
local.set $__intern_var
end
local.get $__intern_var
local.set $pa

;;inc
local.get $pa
local.get $pa
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;papcall
local.get $pa
call $__copy_pap
local.get $pa
i32.const 16
i32.add
local.get $pa
i32.const 12
i32.add
i32.load
i32.const 4
i32.mul
local.get $m
i32.store
local.get $pa
i32.const 12
i32.add
local.get $pa
i32.const 12
i32.add
i32.load
i32.const 1
i32.add
i32.store
local.get $pa
i32.const 12
i32.add
i32.load
local.get $pa
i32.const 8
i32.add
i32.load
call $__nb_args
i32.eq
if
local.get $pa
call $__exec_pap
local.set $__intern_var
else
local.get $pa
local.set $__intern_var
end
local.get $__intern_var
local.set $pa

;;ret
local.get $pa
return
)
(func $fun_main (export "main")(result i32)
(local $r2 i32)
(local $__intern_var i32)
(local $r1 i32)
(local $m i32)
(local $n i32)

;;let

;;num
i32.const 5
call $__make_num
local.set $n

;;let

;;num
i32.const 2
call $__make_num
local.set $m

;;let

;;pap
i32.const 0
call $__make_pap
local.set $__intern_var
local.get $__intern_var
i32.const 16
i32.add
local.get $n
i32.store
local.get $__intern_var
i32.const 12
i32.add
i32.const 1
i32.store
local.get $__intern_var
local.set $r1

;;let

;;papcall
local.get $r1
call $__copy_pap
local.get $r1
i32.const 16
i32.add
local.get $r1
i32.const 12
i32.add
i32.load
i32.const 4
i32.mul
local.get $m
i32.store
local.get $r1
i32.const 12
i32.add
local.get $r1
i32.const 12
i32.add
i32.load
i32.const 1
i32.add
i32.store
local.get $r1
i32.const 12
i32.add
i32.load
local.get $r1
i32.const 8
i32.add
i32.load
call $__nb_args
i32.eq
if
local.get $r1
call $__exec_pap
local.set $__intern_var
else
local.get $r1
local.set $__intern_var
end
local.get $__intern_var
local.set $r2

;;ret
local.get $r2
return
)
(func $fun_getpap (export "getpap")(result i32)
(local $r i32)
(local $n i32)
(local $__intern_var i32)
(local $a i32)

;;let

;;num
i32.const 1
call $__make_num
local.set $n

;;let

;;pap
i32.const 1
call $__make_pap
local.set $__intern_var
local.get $__intern_var
local.set $a

;;let

;;papcall
local.get $a
call $__copy_pap
local.get $a
i32.const 16
i32.add
local.get $a
i32.const 12
i32.add
i32.load
i32.const 4
i32.mul
local.get $n
i32.store
local.get $a
i32.const 12
i32.add
local.get $a
i32.const 12
i32.add
i32.load
i32.const 1
i32.add
i32.store
local.get $a
i32.const 12
i32.add
i32.load
local.get $a
i32.const 8
i32.add
i32.load
call $__nb_args
i32.eq
if
local.get $a
call $__exec_pap
local.set $__intern_var
else
local.get $a
local.set $__intern_var
end
local.get $__intern_var
local.set $r

;;ret
local.get $r
return
)
(func $fun_notpap (export "notpap")(result i32)
(local $__intern_var i32)
(local $r i32)

;;let

;;pap
i32.const 8
call $__make_pap
local.set $__intern_var
local.get $__intern_var
local.set $r

;;ret
local.get $r
return
)
)
