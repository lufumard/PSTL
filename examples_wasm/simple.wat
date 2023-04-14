(module
  (func $i (import "imports" "imported_func") (param f32))
  (func $x (import "imports" "imported_func") (param i32))
  (func (export "exported_func")
    f32.const 21.5
    call $i
  )
)
