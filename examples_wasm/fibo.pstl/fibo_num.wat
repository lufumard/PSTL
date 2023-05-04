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
local.get $var
i32.const 4
i32.add
local.get $var
i32.const 4
i32.add
i32.load
i32.const 1
i32.sub
i32.store
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
(func $fun_fibo (export "fibo")(param $n i32) (result i32)
(local $r i32)
(local $m1 i32)
(local $a i32)
(local $m2 i32)
(local $__intern_var i32)
(local $m i32)
(local $y i32)
(local $x i32)
i32.const 1
call $__make_num
local.set $m1
local.get $n
i32.const 4
i32.add
local.get $n
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
i32.store
local.get $m1
i32.const 4
i32.add
local.get $m1
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
i32.store
local.get $n
i32.const 8
i32.add
i32.load
local.get $m1
i32.const 8
i32.add
i32.load
i32.le_s
(if (then
i32.const 1
call $__make_no_arg
local.set $__intern_var
) (else
i32.const 0
call $__make_no_arg
local.set $__intern_var
))
local.get $__intern_var
local.set $a
(block $__case0
(block $__case1
local.get $a
i32.load
(br_table 
$__case1 $__case0 )
)
local.get $a
i32.const 4
i32.add
local.get $a
i32.const 4
i32.add
i32.load
i32.const 1
i32.sub
i32.store
local.get $n
i32.const 4
i32.add
local.get $n
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
i32.store
local.get $n
i32.const 8
i32.add
i32.load
local.get $m1
i32.const 8
i32.add
i32.load
i32.sub
call $__make_num
local.set $x
i32.const 2
call $__make_num
local.set $m2
local.get $n
i32.const 4
i32.add
local.get $n
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
i32.store
local.get $n
i32.const 8
i32.add
i32.load
local.get $m2
i32.const 8
i32.add
i32.load
i32.sub
call $__make_num
local.set $y
local.get $x
call $fun_fibo
local.set $m
local.get $x
i32.const 4
i32.add
local.get $x
i32.const 4
i32.add
i32.load
i32.const 1
i32.sub
i32.store
local.get $y
call $fun_fibo
local.set $n
local.get $y
i32.const 4
i32.add
local.get $y
i32.const 4
i32.add
i32.load
i32.const 1
i32.sub
i32.store
local.get $n
i32.const 4
i32.add
local.get $n
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
i32.store
local.get $m
i32.const 8
i32.add
i32.load
local.get $n
i32.const 8
i32.add
i32.load
i32.add
call $__make_num
local.set $r
local.get $r
return
)
local.get $a
i32.const 4
i32.add
local.get $a
i32.const 4
i32.add
i32.load
i32.const 1
i32.sub
i32.store
local.get $m1
return
)
)
