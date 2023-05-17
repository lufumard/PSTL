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
    (block $__case14
    (block $__case15
    (block $__case16
    local.get $id
  (br_table 
$__case16 $__case15 $__case14 $__case13 $__case12 $__case11 $__case10 $__case9 $__case8 $__case7 $__case6 $__case5 $__case4 $__case3 $__case2 $__case1 $__case0 )
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
    i32.const 1 
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
   i32.load
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
 
   local.get $args_rest
   if
     (loop $set_arg
       local.get $loc_arg_pap
       local.get $loc_arg_var
       i32.load
       i32.store
 
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
local.get $pap
i32.const 8
i32.add
i32.load
br_table 
$__case16 $__case15 $__case14 $__case13 $__case12 $__case11 $__case10 $__case9 $__case8 $__case7 $__case6 $__case5 $__case4 $__case3 $__case2 $__case1 $__case0 )
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
local.get $p_0
call $__dec
local.get $p_1
call $__dec
local.get $pap
call $__dec
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
local.get $p_0
call $__dec
local.get $p_1
call $__dec
local.get $pap
call $__dec
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
local.get $p_0
call $__dec
local.get $p_1
call $__dec
local.get $pap
call $__dec
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
local.get $p_0
call $__dec
local.get $p_1
call $__dec
local.get $pap
call $__dec
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
local.get $p_0
call $__dec
local.get $p_1
call $__dec
local.get $pap
call $__dec
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
local.get $p_0
call $__dec
local.get $p_1
call $__dec
local.get $pap
call $__dec
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
local.get $p_0
call $__dec
local.get $p_1
call $__dec
local.get $pap
call $__dec
return
)
(i32.add (local.get $pap) (i32.const 16))
i32.load
local.set $p_0
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
local.get $p_0
call $__dec
local.get $p_1
call $__dec
local.get $pap
call $__dec
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
local.get $p_0
call $__dec
local.get $p_1
call $__dec
local.get $pap
call $__dec
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
local.get $p_0
call $__dec
local.get $p_1
call $__dec
local.get $pap
call $__dec
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
local.get $p_0
call $__dec
local.get $p_1
call $__dec
local.get $pap
call $__dec
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
local.get $p_0
call $__dec
local.get $p_1
call $__dec
local.get $pap
call $__dec
return
)
(i32.add (local.get $pap) (i32.const 16))
i32.load
call $fun_head
local.get $pap
call $__dec
return
)
(i32.add (local.get $pap) (i32.const 16))
i32.load
call $fun_tail
local.get $pap
call $__dec
return
)
(i32.add (local.get $pap) (i32.const 16))
i32.load
call $fun_length
local.get $pap
call $__dec
return
)
(i32.add (local.get $pap) (i32.const 16))
i32.load
(i32.add (local.get $pap) (i32.const 20))
i32.load
call $fun_fmap
local.get $pap
call $__dec
return
)
(func $__dec (param $var i32)
 (local $args_left i32)
 (local $ref i32)
 (local $arg i32)
 (i32.add (local.get $var) (i32.const 4))
 i32.load
 local.tee $ref
 if
  local.get $var
  local.get $ref
  i32.const 1
  i32.sub
  call $__set_ref
  local.get $ref
  i32.eqz
  if
    local.get $var
    i32.load
    i32.const 5
    i32.eq
    if
      (i32.add (local.get $var) (i32.const 12))
      i32.load
      local.set $args_left
      (i32.add (local.get $var) (i32.const 16))
      local.set $arg
      (block $dec_end
        (loop $dec_loop
          local.get $args_left
          i32.eqz
          br_if $dec_end
          local.get $arg
          call $__dec
          (i32.sub (local.get $args_left) (i32.const 1))
          local.tee $args_left
          br $dec_loop
        )
      )
    end
    local.get $var
    i32.load
    i32.const 3
    i32.eq
    if ;; si de type LIST
      (i32.add (local.get $var) (i32.const 8)) ;; @@arg 1
      i32.load   ;; @arg 1
      call $__dec;; dec arg 1
      (i32.add (local.get $var) (i32.const 12)) ;; @@arg 2
      i32.load   ;; @arg 2
      call $__dec;; dec arg 2
    end
  end
 end
)
(func $fun_liste (export "liste")(result i32)
(local $var_n3 i32)
(local $var_l1 i32)
(local $var_n5 i32)
(local $var_l5 i32)
(local $var_nil i32)
(local $var_n2 i32)
(local $__intern_var i32)
(local $var_l4 i32)
(local $var_l2 i32)
(local $var_l3 i32)
(local $var_n1 i32)
(local $var_n4 i32)

;;let

;;num
i32.const 5
call $__make_num
local.set $var_n5

;;let

;;ctor
i32.const 2
call $__make_no_arg
local.set $var_nil

;;let

;;ctor
local.get $var_n5
local.get $var_nil
call $__make_list
local.set $var_l5

;;let

;;num
i32.const 4
call $__make_num
local.set $var_n4

;;let

;;ctor
local.get $var_n4
local.get $var_l5
call $__make_list
local.set $var_l4

;;let

;;num
i32.const 3
call $__make_num
local.set $var_n3

;;let

;;ctor
local.get $var_n3
local.get $var_l4
call $__make_list
local.set $var_l3

;;let

;;num
i32.const 2
call $__make_num
local.set $var_n2

;;let

;;ctor
local.get $var_n2
local.get $var_l3
call $__make_list
local.set $var_l2

;;let

;;num
i32.const 1
call $__make_num
local.set $var_n1

;;let

;;ctor
local.get $var_n1
local.get $var_l2
call $__make_list
return
)
(func $fun_head (export "head")(param $var_l i32) (result i32)
(local $__intern_var i32)
(local $var_h i32)

;;let

;;proj
local.get $var_l
i32.const 8
i32.add
i32.load
local.set $var_h

;;inc
local.get $var_h
local.get $var_h
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;ret
local.get $var_h
return
)
(func $fun_tail (export "tail")(param $var_l i32) (result i32)
(local $var_h i32)
(local $__intern_var i32)

;;let

;;proj
local.get $var_l
i32.const 12
i32.add
i32.load
local.set $var_h

;;inc
local.get $var_h
local.get $var_h
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;ret
local.get $var_h
return
)
(func $fun_nil (export "nil")(result i32)
(local $var_r i32)
(local $__intern_var i32)

;;let

;;ctor
i32.const 2
call $__make_no_arg
return
)
(func $fun_length (export "length")(param $var_l i32) (result i32)
(local $var_len2 i32)
(local $__intern_var i32)
(local $var_h i32)
(local $var_t i32)
(local $var_len1 i32)
(local $w1 i32)
(local $var_r i32)

;;case
(block $__case0
(block $__case1
(block $__case2
(block $__case3
(block $__case4
local.get $var_l
i32.load
(br_table 
$__case4 $__case3 $__case2 $__case1 $__case0 )
)

;;dec
local.get $var_l
call $__dec

;;let

;;num
i32.const 1
call $__make_num
return
)

;;dec
local.get $var_l
call $__dec

;;let

;;num
i32.const 1
call $__make_num
return
)

;;dec
local.get $var_l
call $__dec

;;let

;;num
i32.const 0
call $__make_num
return
)

;;let

;;fncall
local.get $var_l
call $fun_head
local.set $var_h

;;let

;;fncall
local.get $var_h
call $fun_length
local.set $var_len1

;;let

;;fncall
local.get $var_l
call $fun_tail
local.set $var_t

;;dec
local.get $var_l
call $__dec

;;let

;;fncall
local.get $var_t
call $fun_length
local.set $var_len2

;;let

;;fncall
local.get $var_len1
i32.const 8
i32.add
i32.load
local.get $var_len2
i32.const 8
i32.add
i32.load
i32.add
call $__make_num
local.get $var_len1
call $__dec
local.get $var_len2
call $__dec
return
)

;;let

;;reset
local.get $var_l
call $__reset
local.set $w1

;;let

;;reuse
local.get $w1
local.get $w1
i32.load
local.tee $__intern_var
i32.const 4
i32.eq
i32.and
if
local.get $w1
i32.const 4
i32.store
local.get $w1
i32.const 4
i32.add
i32.const 1
i32.store
local.get $w1
i32.load
i32.const 8
i32.add
i32.const 1
i32.store
else

;;num
i32.const 1
call $__make_num
local.set $w1
end
local.get $w1
return
)
(func $fun_liste1 (export "liste1")(result i32)
(local $__intern_var i32)
(local $var_l1 i32)
(local $var_l0 i32)
(local $var_nil i32)
(local $var_l2 i32)
(local $var_n1 i32)

;;let

;;num
i32.const 1
call $__make_num
local.set $var_n1

;;let

;;ctor
i32.const 2
call $__make_no_arg
local.set $var_nil

;;inc
local.get $var_n1
local.get $var_n1
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;ctor
local.get $var_n1
local.get $var_nil
call $__make_list
local.set $var_l2

;;inc
local.get $var_n1
local.get $var_n1
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;ctor
local.get $var_n1
local.get $var_l2
call $__make_list
local.set $var_l1

;;let

;;ctor
local.get $var_n1
local.get $var_l1
call $__make_list
return
)
(func $fun_len_liste1 (export "len_liste1")(result i32)
(local $var_l i32)
(local $__intern_var i32)
(local $var_r i32)

;;let

;;fncall
call $fun_liste1
local.set $var_l

;;let

;;fncall
local.get $var_l
call $fun_length
return
)
(func $fun_len_liste (export "len_liste")(result i32)
(local $__intern_var i32)
(local $var_r i32)
(local $var_l i32)

;;let

;;fncall
call $fun_liste
local.set $var_l

;;let

;;fncall
local.get $var_l
call $fun_length
return
)
(func $fun_fmap (export "fmap")(param $var_f i32) (param $var_l i32) (result i32)
(local $var_fh i32)
(local $var_ft i32)
(local $var_r i32)
(local $var_h i32)
(local $var_t i32)
(local $var_rl i32)
(local $w1 i32)
(local $__intern_var i32)

;;case
(block $__case0
(block $__case1
(block $__case2
(block $__case3
(block $__case4
local.get $var_l
i32.load
(br_table 
$__case4 $__case3 $__case2 $__case1 $__case0 )
)

;;let
local.get $var_f
call $__copy_pap
local.tee $__intern_var
local.get $var_f
call $__dec
i32.const 12
i32.add
(i32.add (local.get $__intern_var) (i32.const 12))
i32.load
i32.const 1
i32.add
i32.store
(i32.add (local.get $__intern_var) (i32.const 12))
i32.load
i32.const 4
i32.mul
local.get $__intern_var
i32.add
i32.const 12
i32.add
local.get $var_l
i32.store
(i32.add (local.get $__intern_var) (i32.const 12))
i32.load
(i32.add (local.get $__intern_var) (i32.const 8))
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

;;let
local.get $var_f
call $__copy_pap
local.tee $__intern_var
local.get $var_f
call $__dec
i32.const 12
i32.add
(i32.add (local.get $__intern_var) (i32.const 12))
i32.load
i32.const 1
i32.add
i32.store
(i32.add (local.get $__intern_var) (i32.const 12))
i32.load
i32.const 4
i32.mul
local.get $__intern_var
i32.add
i32.const 12
i32.add
local.get $var_l
i32.store
(i32.add (local.get $__intern_var) (i32.const 12))
i32.load
(i32.add (local.get $__intern_var) (i32.const 8))
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

;;ret
local.get $var_l
return
)

;;let

;;fncall
local.get $var_l
call $fun_head
local.set $var_h

;;inc
local.get $var_f
local.get $var_f
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;fncall
local.get $var_f
local.get $var_h
call $fun_fmap
local.set $var_fh

;;let

;;fncall
local.get $var_l
call $fun_tail
local.set $var_t

;;let

;;reset
local.get $var_l
call $__reset
local.set $w1

;;let

;;fncall
local.get $var_f
local.get $var_t
call $fun_fmap
local.set $var_ft

;;let

;;reuse
local.get $w1
local.get $w1
i32.load
local.tee $__intern_var
i32.const 3
i32.eq
local.get $__intern_var
i32.const 3
i32.le_s
i32.or
i32.and
if
local.get $w1
i32.const 3
i32.store
local.get $w1
i32.const 4
i32.add
i32.const 1
i32.store
local.get $w1
i32.load
i32.const 8
i32.add
local.get $var_fh
i32.store
local.get $w1
i32.const 12
i32.add
local.get $var_ft
i32.store
else
local.get $var_fh
local.get $var_ft
call $__make_list
local.set $w1
end
local.get $w1
return
)

;;let
local.get $var_f
call $__copy_pap
local.tee $__intern_var
local.get $var_f
call $__dec
i32.const 12
i32.add
(i32.add (local.get $__intern_var) (i32.const 12))
i32.load
i32.const 1
i32.add
i32.store
(i32.add (local.get $__intern_var) (i32.const 12))
i32.load
i32.const 4
i32.mul
local.get $__intern_var
i32.add
i32.const 12
i32.add
local.get $var_l
i32.store
(i32.add (local.get $__intern_var) (i32.const 12))
i32.load
(i32.add (local.get $__intern_var) (i32.const 8))
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
(func $fun_papadd1 (export "papadd1")(result i32)
(local $var_n i32)
(local $__intern_var i32)
(local $var_r i32)
(local $var_l i32)
(local $var_f i32)

;;let

;;num
i32.const 1
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
call $fun_liste1
local.set $var_l

;;let

;;fncall
local.get $var_f
local.get $var_l
call $fun_fmap
return
)
(func $fun_papbool (export "papbool")(result i32)
(local $var_l1 i32)
(local $var_r i32)
(local $var_t i32)
(local $__intern_var i32)
(local $var_fun i32)
(local $var_f i32)
(local $var_l2 i32)

;;let

;;ctor
i32.const 0
call $__make_no_arg
local.set $var_f

;;let

;;ctor
i32.const 1
call $__make_no_arg
local.set $var_t

;;inc
local.get $var_t
local.get $var_t
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;ctor
local.get $var_t
local.get $var_f
call $__make_list
local.set $var_l2

;;let

;;ctor
local.get $var_t
local.get $var_l2
call $__make_list
local.set $var_l1

;;let

;;pap
i32.const 7
call $__make_pap
local.set $__intern_var
local.get $__intern_var
local.set $var_fun

;;let

;;fncall
local.get $var_fun
local.get $var_l1
call $fun_fmap
return
)
(func $fun_hl (export "hl")(result i32)
(local $var_h i32)
(local $__intern_var i32)
(local $var_l i32)

;;let

;;fncall
call $fun_liste
local.set $var_l

;;let

;;fncall
local.get $var_l
call $fun_head
local.set $var_h

;;dec
local.get $var_l
call $__dec

;;ret
local.get $var_h
return
)
)
