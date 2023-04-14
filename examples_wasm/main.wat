(module
    (import "env" "helloworld" (func $helloworld))
    (func $main
        call $helloworld
    )
    (start $main)
)