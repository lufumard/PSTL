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
(func $fun_swap (export "swap")(param $xs i32) (result i32)
(local $w3 i32)
(local $w1 i32)
(local $__intern_var i32)
(local $h i32)
(local $t1 i32)
(local $w2 i32)
(local $r i32)
(local $w4 i32)

;;case
(block $__case0
(block $__case1
(block $__case2
(block $__case3
(block $__case4
local.get $xs
i32.load
(br_table 
$__case4 $__case3 $__case2 $__case1 $__case0 )
)

;;ret
local.get $xs
return
)

;;ret
local.get $xs
return
)

;;ret
local.get $xs
return
)

;;let

;;proj
local.get $xs
i32.const 12
i32.add
i32.load
local.set $t1

;;inc
local.get $t1
local.get $t1
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;case
(block $__case0
(block $__case1
(block $__case2
(block $__case3
(block $__case4
local.get $t1
i32.load
(br_table 
$__case4 $__case3 $__case2 $__case1 $__case0 )
)

;;let

;;proj
local.get $xs
i32.const 8
i32.add
i32.load
local.set $h

;;inc
local.get $h
local.get $h
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;reset
local.get $xs
call $__reset
local.set $w1

;;let

;;reuse
local.get $w1
local.get $w1
i32.load
local.set $__intern_var
local.get $__intern_var
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
local.get $t1
i32.store
local.get $w1
i32.const 12
i32.add
local.get $h
i32.store
else
local.get $t1
local.get $h
call $__make_list
local.set $w1
end
local.get $w1
local.set $r

;;ret
local.get $r
return
)

;;let

;;proj
local.get $xs
i32.const 8
i32.add
i32.load
local.set $h

;;inc
local.get $h
local.get $h
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;reset
local.get $xs
call $__reset
local.set $w2

;;let

;;reuse
local.get $w2
local.get $w2
i32.load
local.set $__intern_var
local.get $__intern_var
i32.const 3
i32.eq
local.get $__intern_var
i32.const 3
i32.le_s
i32.or
i32.and
if
local.get $w2
i32.const 3
i32.store
local.get $w2
i32.const 4
i32.add
i32.const 1
i32.store
local.get $w2
i32.load
i32.const 8
i32.add
local.get $t1
i32.store
local.get $w2
i32.const 12
i32.add
local.get $h
i32.store
else
local.get $t1
local.get $h
call $__make_list
local.set $w2
end
local.get $w2
local.set $r

;;ret
local.get $r
return
)

;;dec
local.get $t1
local.get $t1
i32.const 4
i32.add
i32.load
i32.const 1
i32.sub
call $__set_ref

;;ret
local.get $xs
return
)

;;let

;;proj
local.get $xs
i32.const 8
i32.add
i32.load
local.set $h

;;inc
local.get $h
local.get $h
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;reset
local.get $xs
call $__reset
local.set $w3

;;let

;;reuse
local.get $w3
local.get $w3
i32.load
local.set $__intern_var
local.get $__intern_var
i32.const 3
i32.eq
local.get $__intern_var
i32.const 3
i32.le_s
i32.or
i32.and
if
local.get $w3
i32.const 3
i32.store
local.get $w3
i32.const 4
i32.add
i32.const 1
i32.store
local.get $w3
i32.load
i32.const 8
i32.add
local.get $t1
i32.store
local.get $w3
i32.const 12
i32.add
local.get $h
i32.store
else
local.get $t1
local.get $h
call $__make_list
local.set $w3
end
local.get $w3
local.set $r

;;ret
local.get $r
return
)

;;let

;;proj
local.get $xs
i32.const 8
i32.add
i32.load
local.set $h

;;inc
local.get $h
local.get $h
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;reset
local.get $xs
call $__reset
local.set $w4

;;let

;;reuse
local.get $w4
local.get $w4
i32.load
local.set $__intern_var
local.get $__intern_var
i32.const 3
i32.eq
local.get $__intern_var
i32.const 3
i32.le_s
i32.or
i32.and
if
local.get $w4
i32.const 3
i32.store
local.get $w4
i32.const 4
i32.add
i32.const 1
i32.store
local.get $w4
i32.load
i32.const 8
i32.add
local.get $t1
i32.store
local.get $w4
i32.const 12
i32.add
local.get $h
i32.store
else
local.get $t1
local.get $h
call $__make_list
local.set $w4
end
local.get $w4
local.set $r

;;ret
local.get $r
return
)

;;ret
local.get $xs
return
)
)
