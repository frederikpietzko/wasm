(module
    (import "wasi_unstable" "fd_write"
        (func $fd_write
            (param i32 i32 i32 i32)
            (result i32)))
    (import "wasi_unstable" "fd_read"
        (func $fd_read
              (param i32 i32 i32 i32)
              (result i32)))

    (func $add (param $p1 i32) (param $p2 i32) (result i32)
        local.get $p1
        local.get $p2
        i32.add
    )
    (func $folded_add (param $p1 i32) (param $p2 i32) (result i32)
        (i32.add (local.get $p1) (local.get $p2))
    )

    (export "add" (func $add))
    (export "folded_add" (func $folded_add))

    (memory 1)
    (export "memory" (memory 0))
    (data (i32.const 8) "hello\n")

    (func $main (export "main")
        (i32.store (i32.const 0) (i32.const 8))
        (i32.store (i32.const 4) (i32.const 6))
        (call $fd_write
            (i32.const 1) ;; stdout
            (i32.const 0) ;; pointer to io vector
            (i32.const 1) ;; vector length
            (i32.const 0)) ;; nwritten
        drop
    )
    (func $echo (export "echo")
        (i32.store (i32.const 4) (i32.const 12))
        (i32.store (i32.const 8) (i32.const 100))
        (call $fd_read
            (i32.const 0) ;; 0 for stdin
            (i32.const 4) ;; *iovs
            (i32.const 1) ;; iovs_len
            (i32.const 8) ;; nread
        )
        drop
        (call $fd_write
            (i32.const 1) ;; 1 for stdout
            (i32.const 4) ;; *iovs
            (i32.const 1) ;; iovs_len
            (i32.const 0) ;; nwritten
        )
        drop
    )

)
