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
    local.get $id
  (br_table 
$__case14 $__case13 $__case12 $__case11 $__case10 $__case9 $__case8 $__case7 $__case6 $__case5 $__case4 $__case3 $__case2 $__case1 $__case0 )
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
local.get $pap
i32.const 8
i32.add
i32.load
br_table 
$__case14 $__case13 $__case12 $__case11 $__case10 $__case9 $__case8 $__case7 $__case6 $__case5 $__case4 $__case3 $__case2 $__case1 $__case0 )
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
call $fun_fibo
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
call $fun_main
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
(func $fun_fibo (export "fibo")(param $var_n i32) (result i32)
(local $var_y i32)
(local $__intern_var i32)
(local $var_f2 i32)
(local $var_m1 i32)
(local $var_x i32)
(local $var_r i32)
(local $var_m i32)
(local $var_m2 i32)
(local $var_f1 i32)
(local $var_a i32)

;;let

;;num
i32.const 1
call $__make_num
local.set $var_m

;;inc
local.get $var_n
local.get $var_n
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;fncall
local.get $var_n
i32.const 8
i32.add
i32.load
local.get $var_m
i32.const 8
i32.add
i32.load
i32.le_s
call $__make_no_arg
local.get $var_n
call $__dec
local.get $var_m
call $__dec
local.set $var_a

;;case
(block $__case0
(block $__case1
local.get $var_a
i32.load
(br_table 
$__case1 $__case0 )
)

;;dec
local.get $var_a
call $__dec

;;let

;;num
i32.const 1
call $__make_num
local.set $var_m1

;;inc
local.get $var_n
local.get $var_n
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;fncall
local.get $var_n
i32.const 8
i32.add
i32.load
local.get $var_m1
i32.const 8
i32.add
i32.load
i32.sub
call $__make_num
local.get $var_n
call $__dec
local.get $var_m1
call $__dec
local.set $var_x

;;let

;;fncall
local.get $var_x
call $fun_fibo
local.set $var_f1

;;dec
local.get $var_x
call $__dec

;;let

;;num
i32.const 2
call $__make_num
local.set $var_m2

;;inc
local.get $var_n
local.get $var_n
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;fncall
local.get $var_n
i32.const 8
i32.add
i32.load
local.get $var_m2
i32.const 8
i32.add
i32.load
i32.sub
call $__make_num
local.get $var_n
call $__dec
local.get $var_m2
call $__dec
local.set $var_y

;;let

;;fncall
local.get $var_y
call $fun_fibo
local.set $var_f2

;;dec
local.get $var_y
call $__dec

;;let

;;fncall
local.get $var_f1
i32.const 8
i32.add
i32.load
local.get $var_f2
i32.const 8
i32.add
i32.load
i32.add
call $__make_num
local.get $var_f1
call $__dec
local.get $var_f2
call $__dec
return
)

;;dec
local.get $var_a
call $__dec

;;inc
local.get $var_n
local.get $var_n
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;ret
local.get $var_n
return
)
(func $fun_main10 (export "main10")(result i32)
(local $var_n i32)
(local $__intern_var i32)
(local $var_r i32)

;;let

;;num
i32.const 2
call $__make_num
local.set $var_n

;;let

;;fncall
local.get $var_n
call $fun_main
local.set $var_r

;;dec
local.get $var_n
call $__dec

;;ret
local.get $var_r
return
)
(func $fun_main2 (export "main2")(result i32)
(local $var_n i32)
(local $var_r i32)
(local $__intern_var i32)

;;let

;;num
i32.const 2
call $__make_num
local.set $var_n

;;let

;;fncall
local.get $var_n
call $fun_fibo
local.set $var_r

;;dec
local.get $var_n
call $__dec

;;ret
local.get $var_r
return
)
(func $fun_main (export "main")(param $var_n i32) (result i32)
(local $var_x i32)
(local $var_r i32)
(local $__intern_var i32)
(local $var_m i32)

;;let

;;num
i32.const 5
call $__make_num
local.set $var_m

;;inc
local.get $var_n
local.get $var_n
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;fncall
local.get $var_n
i32.const 8
i32.add
i32.load
local.get $var_m
i32.const 8
i32.add
i32.load
i32.mul
call $__make_num
local.get $var_n
call $__dec
local.get $var_m
call $__dec
local.set $var_x

;;let

;;fncall
local.get $var_x
call $fun_fibo
local.set $var_r

;;dec
local.get $var_x
call $__dec

;;ret
local.get $var_r
return
)
(func $fun_main7 (export "main7")(result i32)
(local $var_n i32)
(local $__intern_var i32)
(local $var_r i32)

;;let

;;num
i32.const 7
call $__make_num
local.set $var_n

;;let

;;fncall
local.get $var_n
call $fun_fibo
local.set $var_r

;;dec
local.get $var_n
call $__dec

;;ret
local.get $var_r
return
)
(func $fun_main15 (export "main15")(result i32)
(local $var_r i32)
(local $var_n i32)
(local $__intern_var i32)

;;let

;;num
i32.const 15
call $__make_num
local.set $var_n

;;let

;;fncall
local.get $var_n
call $fun_fibo
local.set $var_r

;;dec
local.get $var_n
call $__dec

;;ret
local.get $var_r
return
)
(func $fun_main5 (export "main5")(result i32)
(local $__intern_var i32)
(local $var_r i32)
(local $var_n i32)

;;let

;;num
i32.const 5
call $__make_num
local.set $var_n

;;let

;;fncall
local.get $var_n
call $fun_fibo
local.set $var_r

;;dec
local.get $var_n
call $__dec

;;ret
local.get $var_r
return
)
(func $fun_main35 (export "main35")(result i32)
(local $var_r i32)
(local $__intern_var i32)
(local $var_n i32)

;;let

;;num
i32.const 35
call $__make_num
local.set $var_n

;;let

;;fncall
local.get $var_n
call $fun_fibo
local.set $var_r

;;dec
local.get $var_n
call $__dec

;;ret
local.get $var_r
return
)
(func $fun_main30 (export "main30")(result i32)
(local $var_r i32)
(local $__intern_var i32)
(local $var_n i32)

;;let

;;num
i32.const 30
call $__make_num
local.set $var_n

;;let

;;fncall
local.get $var_n
call $fun_fibo
local.set $var_r

;;dec
local.get $var_n
call $__dec

;;ret
local.get $var_r
return
)
(func $fun_main25 (export "main25")(result i32)
(local $var_n i32)
(local $var_r i32)
(local $__intern_var i32)

;;let

;;num
i32.const 25
call $__make_num
local.set $var_n

;;let

;;fncall
local.get $var_n
call $fun_fibo
local.set $var_r

;;dec
local.get $var_n
call $__dec

;;ret
local.get $var_r
return
)
(func $fun_main20 (export "main20")(result i32)
(local $var_n i32)
(local $__intern_var i32)
(local $var_r i32)

;;let

;;num
i32.const 20
call $__make_num
local.set $var_n

;;let

;;fncall
local.get $var_n
call $fun_fibo
local.set $var_r

;;dec
local.get $var_n
call $__dec

;;ret
local.get $var_r
return
)
)
