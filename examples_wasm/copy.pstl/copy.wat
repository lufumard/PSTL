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
(func $copy (export "copy")(param $x i32) (result i32)
(local $nl2 i32)
(local $l2 i32)
(local $w4 i32)
(local $r i32)
(local $nl1 i32)
(local $n0 i32)
(local $l1 i32)
(local $w3 i32)
(local $w1 i32)
(local $res i32)
(local $w2 i32)
(block $__case0
(block $__case1
(block $__case2
(block $__case3
(block $__case4
local.get $x
i32.load
(br_table 
$__case4 $__case3 $__case2 $__case1 $__case0 )
)
local.set $w1
local.set $r
local.get $r
return
)
local.set $w2
local.set $r
local.get $r
return
)
local.set $w3
local.set $r
local.get $r
return
)
local.get $x
i32.const 8
i32.add
i32.load
local.set $l1
local.get $l1
call $copy
local.set $nl1
local.get $x
i32.const 12
i32.add
i32.load
local.set $l2
local.set $w4
local.get $l2
call $copy
local.set $nl2
local.set $res
local.get $res
return
)
i32.const 0
call $__make_num
local.set $n0
local.get $x
local.get $n0
local.get $x
i32.const 8
i32.add
i32.load
local.get $n0
i32.const 8
i32.add
i32.load
i32.add
call $__make_num
local.set $r
local.get $r
return
)
(func $main0 (export "main0")(result i32)
(local $r i32)
(local $f i32)
i32.const 0
call $__make_no_arg
local.set $f
local.get $f
call $copy
local.set $r
local.get $r
return
)
(func $main1 (export "main1")(result i32)
(local $t i32)
(local $r i32)
i32.const 1
call $__make_no_arg
local.set $t
local.get $t
call $copy
local.set $r
local.get $r
return
)
(func $main2 (export "main2")(result i32)
(local $r i32)
(local $nil i32)
i32.const 2
call $__make_no_arg
local.set $nil
local.get $nil
call $copy
local.set $r
local.get $r
return
)
(func $main4 (export "main4")(result i32)
(local $r i32)
(local $n i32)
i32.const 4
call $__make_num
local.set $n
local.get $n
call $copy
local.set $r
local.get $r
return
)
(func $main3 (export "main3")(result i32)
(local $l2 i32)
(local $l1 i32)
(local $r i32)
(local $n1 i32)
(local $n0 i32)
(local $n2 i32)
i32.const 1
call $__make_num
local.set $n1
i32.const 2
call $__make_num
local.set $n2
local.get $n1
local.get $n2
call $__make_list
local.set $l2
i32.const 0
call $__make_num
local.set $n0
local.get $n0
local.get $l2
call $__make_list
local.set $l1
local.get $l1
call $copy
local.set $r
local.get $r
return
)
)
