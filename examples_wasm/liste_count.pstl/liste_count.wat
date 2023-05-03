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
(func $fun_count (export "count")(param $a i32) (param $b i32) (result i32)
(local $r i32)
(block $__case0
(block $__case1
(block $__case2
(block $__case3
(block $__case4
local.get $a
i32.load
(br_table 
$__case4 $__case3 $__case2 $__case1 $__case0 )
)
(block $__case0
(block $__case1
(block $__case2
(block $__case3
(block $__case4
local.get $b
i32.load
(br_table 
$__case4 $__case3 $__case2 $__case1 $__case0 )
)
i32.const 10
call $__make_num
return
)
i32.const 11
call $__make_num
return
)
i32.const 12
call $__make_num
return
)
i32.const 13
call $__make_num
return
)
i32.const 14
call $__make_num
return
)
(block $__case0
(block $__case1
(block $__case2
(block $__case3
(block $__case4
local.get $b
i32.load
(br_table 
$__case4 $__case3 $__case2 $__case1 $__case0 )
)
i32.const 20
call $__make_num
return
)
i32.const 21
call $__make_num
return
)
i32.const 22
call $__make_num
return
)
i32.const 23
call $__make_num
return
)
i32.const 24
call $__make_num
return
)
(block $__case0
(block $__case1
(block $__case2
(block $__case3
(block $__case4
local.get $b
i32.load
(br_table 
$__case4 $__case3 $__case2 $__case1 $__case0 )
)
i32.const 30
call $__make_num
return
)
i32.const 31
call $__make_num
return
)
i32.const 32
call $__make_num
return
)
i32.const 33
call $__make_num
return
)
i32.const 34
call $__make_num
return
)
(block $__case0
(block $__case1
(block $__case2
(block $__case3
(block $__case4
local.get $b
i32.load
(br_table 
$__case4 $__case3 $__case2 $__case1 $__case0 )
)
i32.const 40
call $__make_num
return
)
i32.const 41
call $__make_num
return
)
i32.const 42
call $__make_num
return
)
i32.const 43
call $__make_num
return
)
i32.const 44
call $__make_num
return
)
(block $__case0
(block $__case1
(block $__case2
(block $__case3
(block $__case4
local.get $b
i32.load
(br_table 
$__case4 $__case3 $__case2 $__case1 $__case0 )
)
i32.const 50
call $__make_num
return
)
i32.const 51
call $__make_num
return
)
i32.const 52
call $__make_num
return
)
i32.const 53
call $__make_num
return
)
i32.const 54
call $__make_num
return
)
)
