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
(func $fun_fibo_liste (export "fibo_liste")(param $n i32) (param $l i32) (result i32)
(local $m1 i32)
(local $b i32)
(local $__intern_var i32)
(local $r i32)
(local $c i32)
(local $a i32)

;;let

;;num
i32.const 1
call $__make_num
local.set $m1

;;inc
local.get $n
local.get $n
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;inc
local.get $m1
local.get $m1
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;fncall
local.get $n
i32.const 8
i32.add
i32.load
local.get $m1
i32.const 8
i32.add
i32.load
i32.lt_s
call $__make_no_arg
local.set $a

;;case
(block $__case0
(block $__case1
local.get $a
i32.load
(br_table 
$__case1 $__case0 )
)

;;let

;;proj
local.get $l
i32.const 8
i32.add
i32.load
local.set $a

;;let

;;proj
local.get $l
i32.const 12
i32.add
i32.load
local.set $b

;;inc
local.get $a
local.get $a
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;inc
local.get $b
local.get $b
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;fncall
local.get $a
i32.const 8
i32.add
i32.load
local.get $b
i32.const 8
i32.add
i32.load
i32.add
call $__make_num
local.set $c

;;inc
local.get $b
local.get $b
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;ctor
local.get $b
local.get $c
call $__make_list
local.set $l

;;inc
local.get $n
local.get $n
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;fncall
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
local.set $n

;;let

;;fncall
local.get $n
local.get $l
call $fun_fibo_liste
local.set $r

;;ret
local.get $r
return
)

;;dec
local.get $a
local.get $a
i32.const 4
i32.add
i32.load
i32.const 1
i32.sub
call $__set_ref

;;let

;;proj
local.get $l
i32.const 8
i32.add
i32.load
local.set $r

;;inc
local.get $r
local.get $r
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;ret
local.get $r
return
)
(func $fun_fibo (export "fibo")(param $n i32) (result i32)
(local $l i32)
(local $m0 i32)
(local $__intern_var i32)
(local $m1 i32)
(local $r i32)

;;let

;;num
i32.const 1
call $__make_num
local.set $m1

;;let

;;num
i32.const 0
call $__make_num
local.set $m0

;;let

;;ctor
local.get $m0
local.get $m1
call $__make_list
local.set $l

;;let

;;fncall
local.get $n
local.get $l
call $fun_fibo_liste
local.set $r

;;ret
local.get $r
return
)
(func $fun_fibo25 (export "fibo25")(result i32)
(local $r i32)
(local $__intern_var i32)
(local $n i32)

;;let

;;num
i32.const 25
call $__make_num
local.set $n

;;let

;;fncall
local.get $n
call $fun_fibo
local.set $r

;;ret
local.get $r
return
)
(func $fun_fibo50 (export "fibo50")(result i32)
(local $n i32)
(local $__intern_var i32)
(local $r i32)

;;let

;;num
i32.const 50
call $__make_num
local.set $n

;;let

;;fncall
local.get $n
call $fun_fibo
local.set $r

;;ret
local.get $r
return
)
(func $fun_fibo75 (export "fibo75")(result i32)
(local $n i32)
(local $__intern_var i32)
(local $r i32)

;;let

;;num
i32.const 75
call $__make_num
local.set $n

;;let

;;fncall
local.get $n
call $fun_fibo
local.set $r

;;ret
local.get $r
return
)
(func $fun_fibo100 (export "fibo100")(result i32)
(local $r i32)
(local $n i32)
(local $__intern_var i32)

;;let

;;num
i32.const 100
call $__make_num
local.set $n

;;let

;;fncall
local.get $n
call $fun_fibo
local.set $r

;;ret
local.get $r
return
)
(func $fun_fibo40 (export "fibo40")(result i32)
(local $n i32)
(local $__intern_var i32)
(local $r i32)

;;let

;;num
i32.const 40
call $__make_num
local.set $n

;;let

;;fncall
local.get $n
call $fun_fibo
local.set $r

;;ret
local.get $r
return
)
(func $fun_fibo41 (export "fibo41")(result i32)
(local $n i32)
(local $__intern_var i32)
(local $r i32)

;;let

;;num
i32.const 41
call $__make_num
local.set $n

;;let

;;fncall
local.get $n
call $fun_fibo
local.set $r

;;ret
local.get $r
return
)
(func $fun_fibo42 (export "fibo42")(result i32)
(local $__intern_var i32)
(local $r i32)
(local $n i32)

;;let

;;num
i32.const 42
call $__make_num
local.set $n

;;let

;;fncall
local.get $n
call $fun_fibo
local.set $r

;;ret
local.get $r
return
)
(func $fun_fibo43 (export "fibo43")(result i32)
(local $r i32)
(local $n i32)
(local $__intern_var i32)

;;let

;;num
i32.const 43
call $__make_num
local.set $n

;;let

;;fncall
local.get $n
call $fun_fibo
local.set $r

;;ret
local.get $r
return
)
(func $fun_fibo44 (export "fibo44")(result i32)
(local $__intern_var i32)
(local $r i32)
(local $n i32)

;;let

;;num
i32.const 44
call $__make_num
local.set $n

;;let

;;fncall
local.get $n
call $fun_fibo
local.set $r

;;ret
local.get $r
return
)
(func $fun_fibo45 (export "fibo45")(result i32)
(local $n i32)
(local $r i32)
(local $__intern_var i32)

;;let

;;num
i32.const 45
call $__make_num
local.set $n

;;let

;;fncall
local.get $n
call $fun_fibo
local.set $r

;;ret
local.get $r
return
)
(func $fun_fibo46 (export "fibo46")(result i32)
(local $n i32)
(local $r i32)
(local $__intern_var i32)

;;let

;;num
i32.const 46
call $__make_num
local.set $n

;;let

;;fncall
local.get $n
call $fun_fibo
local.set $r

;;ret
local.get $r
return
)
)
