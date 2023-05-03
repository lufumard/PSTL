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
(func $fun_liste (export "liste")(result i32)
(local $l4 i32)
(local $l5 i32)
(local $n2 i32)
(local $n3 i32)
(local $n1 i32)
(local $l2 i32)
(local $n5 i32)
(local $n4 i32)
(local $l1 i32)
(local $nil i32)
(local $l3 i32)
i32.const 5
call $__make_num
local.set $n5
i32.const 2
call $__make_no_arg
local.set $nil
local.get $n5
local.get $nil
call $__make_list
local.set $l5
i32.const 4
call $__make_num
local.set $n4
local.get $n4
local.get $l5
call $__make_list
local.set $l4
i32.const 3
call $__make_num
local.set $n3
local.get $n3
local.get $l4
call $__make_list
local.set $l3
i32.const 2
call $__make_num
local.set $n2
local.get $n2
local.get $l3
call $__make_list
local.set $l2
i32.const 1
call $__make_num
local.set $n1
local.get $n1
local.get $l2
call $__make_list
return
)
(func $fun_head (export "head")(param $l i32) (result i32)
(local $h i32)
local.get $l
i32.const 8
i32.add
i32.load
local.set $h
local.get $h
return
)
(func $fun_tail (export "tail")(param $l i32) (result i32)
(local $h i32)
local.get $l
i32.const 12
i32.add
i32.load
local.set $h
local.get $h
return
)
(func $fun_first (export "first")(param $l i32) (result i32)
(local $l i32)
(local $r i32)
(block $__case0
(block $__case1
(block $__case2
(block $__case3
(block $__case4
local.get $l
i32.load
(br_table 
$__case4 $__case3 $__case2 $__case1 $__case0 )
)
local.get $l
return
)
local.get $l
return
)
local.get $l
return
)
local.get $l
call $fun_head
local.set $l
local.get $l
call $fun_first
local.set $r
(block $__case0
(block $__case1
(block $__case2
(block $__case3
(block $__case4
local.get $r
i32.load
(br_table 
$__case4 $__case3 $__case2 $__case1 $__case0 )
)
local.get $r
return
)
local.get $r
return
)
local.get $l
call $fun_tail
local.set $l
local.get $l
call $fun_first
return
)
local.get $r
return
)
local.get $r
return
)
local.get $l
return
)
(func $fun_nil (export "nil")(result i32)
(local $r i32)
i32.const 2
call $__make_no_arg
return
)
(func $fun_last (export "last")(param $l i32) (result i32)
(local $l i32)
(local $r i32)
(block $__case0
(block $__case1
(block $__case2
(block $__case3
(block $__case4
local.get $l
i32.load
(br_table 
$__case4 $__case3 $__case2 $__case1 $__case0 )
)
local.get $l
return
)
local.get $l
return
)
local.get $l
return
)
local.get $l
call $fun_tail
local.set $l
local.get $l
call $fun_last
local.set $r
(block $__case0
(block $__case1
(block $__case2
(block $__case3
(block $__case4
local.get $r
i32.load
(br_table 
$__case4 $__case3 $__case2 $__case1 $__case0 )
)
local.get $r
return
)
local.get $r
return
)
local.get $l
call $fun_head
local.set $l
local.get $l
call $fun_last
return
)
local.get $r
return
)
local.get $r
return
)
local.get $l
return
)
(func $fun_length (export "length")(param $l i32) (result i32)
(local $len1 i32)
(local $len2 i32)
(local $t i32)
(local $h i32)
(local $r i32)
(block $__case0
(block $__case1
(block $__case2
(block $__case3
(block $__case4
local.get $l
i32.load
(br_table 
$__case4 $__case3 $__case2 $__case1 $__case0 )
)
i32.const 1
call $__make_num
return
)
i32.const 1
call $__make_num
return
)
i32.const 0
call $__make_num
return
)
local.get $l
call $fun_head
local.set $h
local.get $h
call $fun_length
local.set $len1
local.get $l
call $fun_tail
local.set $t
local.get $t
call $fun_length
local.set $len2
local.get $len1
i32.const 8
i32.add
i32.load
local.get $len2
i32.const 8
i32.add
i32.load
i32.add
call $__make_num
return
)
i32.const 1
call $__make_num
return
)
(func $fun_liste1 (export "liste1")(result i32)
(local $l0 i32)
(local $l4 i32)
(local $nil i32)
(local $l1 i32)
(local $l3 i32)
(local $l2 i32)
(local $n1 i32)
(local $l5 i32)
i32.const 1
call $__make_num
local.set $n1
i32.const 2
call $__make_no_arg
local.set $nil
local.get $n1
local.get $nil
call $__make_list
local.set $l5
local.get $n1
local.get $l5
call $__make_list
local.set $l4
local.get $n1
local.get $l4
call $__make_list
local.set $l3
local.get $n1
local.get $l3
call $__make_list
local.set $l2
local.get $n1
local.get $l2
call $__make_list
local.set $l1
local.get $n1
local.get $l1
call $__make_list
return
)
)
