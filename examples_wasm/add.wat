(module
  (func $add (param $lhs i32) (param $rhs i32) (result i32)
    local.get $lhs
    local.get $rhs
    i32.add
    i32.const 3
    i32.add
    )
  (export "add" (func $add))
)