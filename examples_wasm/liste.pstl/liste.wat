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
(func $fun_liste (export "liste")(result i32)
(local $l1 i32)
(local $n2 i32)
(local $l3 i32)
(local $n5 i32)
(local $n1 i32)
(local $n3 i32)
(local $l4 i32)
(local $__intern_var i32)
(local $l5 i32)
(local $nil i32)
(local $l2 i32)
(local $n4 i32)

;;let

;;num
i32.const 5
call $__make_num
local.set $n5

;;let

;;ctor
i32.const 2
call $__make_no_arg
local.set $nil

;;let

;;ctor
local.get $n5
local.get $nil
call $__make_list
local.set $l5

;;let

;;num
i32.const 4
call $__make_num
local.set $n4

;;let

;;ctor
local.get $n4
local.get $l5
call $__make_list
local.set $l4

;;let

;;num
i32.const 3
call $__make_num
local.set $n3

;;let

;;ctor
local.get $n3
local.get $l4
call $__make_list
local.set $l3

;;let

;;num
i32.const 2
call $__make_num
local.set $n2

;;let

;;ctor
local.get $n2
local.get $l3
call $__make_list
local.set $l2

;;let

;;num
i32.const 1
call $__make_num
local.set $n1

;;let

;;ctor
local.get $n1
local.get $l2
call $__make_list
local.set $l1

;;ret
local.get $l1
return
)
(func $fun_head (export "head")(param $l i32) (result i32)
(local $__intern_var i32)
(local $h i32)

;;let

;;proj
local.get $l
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

;;ret
local.get $h
return
)
(func $fun_tail (export "tail")(param $l i32) (result i32)
(local $h i32)
(local $__intern_var i32)

;;let

;;proj
local.get $l
i32.const 12
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

;;ret
local.get $h
return
)
(func $fun_nil (export "nil")(result i32)
(local $__intern_var i32)
(local $r i32)

;;let

;;ctor
i32.const 2
call $__make_no_arg
local.set $r

;;ret
local.get $r
return
)
(func $fun_length (export "length")(param $l i32) (result i32)
(local $len2 i32)
(local $t i32)
(local $w1 i32)
(local $r i32)
(local $__intern_var i32)
(local $len1 i32)
(local $h i32)

;;case
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

;;dec
local.get $l
local.get $l
i32.const 4
i32.add
i32.load
i32.const 1
i32.sub
call $__set_ref

;;let

;;num
i32.const 1
call $__make_num
local.set $r

;;ret
local.get $r
return
)

;;dec
local.get $l
local.get $l
i32.const 4
i32.add
i32.load
i32.const 1
i32.sub
call $__set_ref

;;let

;;num
i32.const 1
call $__make_num
local.set $r

;;ret
local.get $r
return
)

;;dec
local.get $l
local.get $l
i32.const 4
i32.add
i32.load
i32.const 1
i32.sub
call $__set_ref

;;let

;;num
i32.const 0
call $__make_num
local.set $r

;;ret
local.get $r
return
)

;;let

;;fncall
local.get $l
call $fun_head
local.set $h

;;let

;;fncall
local.get $h
call $fun_length
local.set $len1

;;let

;;fncall
local.get $l
call $fun_tail
local.set $t

;;dec
local.get $l
local.get $l
i32.const 4
i32.add
i32.load
i32.const 1
i32.sub
call $__set_ref

;;let

;;fncall
local.get $t
call $fun_length
local.set $len2

;;let

;;fncall
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
local.set $r

;;ret
local.get $r
return
)

;;let

;;reset
local.get $l
call $__reset
local.set $w1

;;let

;;reuse
local.get $w1
local.get $w1
i32.load
local.set $__intern_var
local.get $__intern_var
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
local.set $r

;;ret
local.get $r
return
)
(func $fun_liste1 (export "liste1")(result i32)
(local $__intern_var i32)
(local $l1 i32)
(local $l0 i32)
(local $l2 i32)
(local $l3 i32)
(local $nil i32)
(local $l5 i32)
(local $n1 i32)
(local $l4 i32)

;;let

;;num
i32.const 1
call $__make_num
local.set $n1

;;let

;;ctor
i32.const 2
call $__make_no_arg
local.set $nil

;;inc
local.get $n1
local.get $n1
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;ctor
local.get $n1
local.get $nil
call $__make_list
local.set $l5

;;inc
local.get $n1
local.get $n1
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;ctor
local.get $n1
local.get $l5
call $__make_list
local.set $l4

;;inc
local.get $n1
local.get $n1
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;ctor
local.get $n1
local.get $l4
call $__make_list
local.set $l3

;;inc
local.get $n1
local.get $n1
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;ctor
local.get $n1
local.get $l3
call $__make_list
local.set $l2

;;inc
local.get $n1
local.get $n1
i32.const 4
i32.add
i32.load
i32.const 1
i32.add
call $__set_ref

;;let

;;ctor
local.get $n1
local.get $l2
call $__make_list
local.set $l1

;;let

;;ctor
local.get $n1
local.get $l1
call $__make_list
local.set $l0

;;ret
local.get $l0
return
)
)
