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
)
