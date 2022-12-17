# Experiment with allocator_api

I want to test the hypothesis that a custom allocator
can manage a "Protocol" (an enum of message types) faster
that using the global allocator.

Currently compiles, runs, test, and address sanitizer pass,
but valgrind is reporting a couple heap blocks aren't being
released.

## Running

```
wink@3900x 22-12-17T17:41:42.586Z:~/prgs/rust/myrepos/exper_allocator_api (main)
$ cargo run
   Compiling once_cell v1.16.0
   Compiling lazy_static v1.4.0
   Compiling exper_allocator_api v0.3.0 (/home/wink/prgs/rust/myrepos/exper_allocator_api)
    Finished dev [unoptimized + debuginfo] target(s) in 0.51s
     Running `target/debug/exper_allocator_api`
main:+
main inner:+
ma_init:+
ma_init:- len=10
MyAllocator::allocate: p_mut_u8=0x55b2fc866c30
main inner: m=Add { left: 5, right: 6 }
MyAllocator::allocate: p_mut_u8=0x55b2fc866c20
main inner: m=Add { left: 2, right: 4 }
main inner:-
MyAllocator::deallocate: p_mut_u8=0x55b2fc866c20
MyAllocator::deallocate: p_mut_u8=0x55b2fc866c30
main:-
```

## Tests

Passing
```
wink@3900x 22-12-17T17:43:36.135Z:~/prgs/rust/myrepos/exper_allocator_api (main)
$ cargo test
   Compiling lazy_static v1.4.0
   Compiling once_cell v1.16.0
   Compiling exper_allocator_api v0.3.0 (/home/wink/prgs/rust/myrepos/exper_allocator_api)
    Finished test [unoptimized + debuginfo] target(s) in 0.50s
     Running unittests src/lib.rs (target/debug/deps/exper_allocator_api-2283f71710c7153f)

running 1 test
test tests::test_one_allocation ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/exper_allocator_api-3dbcce3e92869130)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests exper_allocator_api

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Valgrind

`valgrind` we are not releasing 2 blocks in the heap:
```
==17977== HEAP SUMMARY:
==17977==     in use at exit: 288 bytes in 2 blocks
==17977==   total heap usage: 15 allocs, 13 frees, 3,565 bytes allocated
```

Full output:
```
wink@3900x 22-12-17T17:44:55.914Z:~/prgs/rust/myrepos/exper_allocator_api (main)
$ valgrind target/debug/exper_allocator_api
==17977== Memcheck, a memory error detector
==17977== Copyright (C) 2002-2022, and GNU GPL'd, by Julian Seward et al.
==17977== Using Valgrind-3.19.0 and LibVEX; rerun with -h for copyright info
==17977== Command: target/debug/exper_allocator_api
==17977== 
main:+
main inner:+
ma_init:+
ma_init:- len=10
MyAllocator::allocate: p_mut_u8=0x4a8c020
main inner: m=Add { left: 5, right: 6 }
MyAllocator::allocate: p_mut_u8=0x4a8c010
main inner: m=Add { left: 2, right: 4 }
main inner:-
MyAllocator::deallocate: p_mut_u8=0x4a8c010
MyAllocator::deallocate: p_mut_u8=0x4a8c020
main:-
==17977== 
==17977== HEAP SUMMARY:
==17977==     in use at exit: 288 bytes in 2 blocks
==17977==   total heap usage: 15 allocs, 13 frees, 3,565 bytes allocated
==17977== 
==17977== LEAK SUMMARY:
==17977==    definitely lost: 0 bytes in 0 blocks
==17977==    indirectly lost: 0 bytes in 0 blocks
==17977==      possibly lost: 0 bytes in 0 blocks
==17977==    still reachable: 288 bytes in 2 blocks
==17977==         suppressed: 0 bytes in 0 blocks
==17977== Rerun with --leak-check=full to see details of leaked memory
==17977== 
==17977== For lists of detected and suppressed errors, rerun with: -s
==17977== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
```

## stanitizer AddressSanitizer

AddressSanitizer succeeds:
```
wink@3900x 22-12-17T17:46:35.936Z:~/prgs/rust/myrepos/exper_allocator_api (main)
$ RUSTFLAGS=-Zsanitizer=address cargo run -Zbuild-std --target x86_64-unknown-linux-gnu
   Compiling compiler_builtins v0.1.85
   Compiling core v0.0.0 (/home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core)
   Compiling libc v0.2.138
   ...
   Compiling lazy_static v1.4.0
   Compiling exper_allocator_api v0.3.0 (/home/wink/prgs/rust/myrepos/exper_allocator_api)
    Finished dev [unoptimized + debuginfo] target(s) in 11.84s
     Running `target/x86_64-unknown-linux-gnu/debug/exper_allocator_api`
main:+
main inner:+
ma_init:+
ma_init:- len=10
MyAllocator::allocate: p_mut_u8=0x60e0000001b0
main inner: m=Add { left: 5, right: 6 }
MyAllocator::allocate: p_mut_u8=0x60e0000001a0
main inner: m=Add { left: 2, right: 4 }
main inner:-
MyAllocator::deallocate: p_mut_u8=0x60e0000001a0
MyAllocator::deallocate: p_mut_u8=0x60e0000001b0
main:-
```

## ltrace

ltrace is reporting 10 mallocs and 10 frees:
```
wink@3900x 22-12-17T17:54:30.766Z:~/prgs/rust/myrepos/exper_allocator_api (main)
$ ltrace -x "malloc+free" target/debug/exper_allocator_api 2>&1 | rg malloc | wc -l
10
wink@3900x 22-12-17T17:54:38.739Z:~/prgs/rust/myrepos/exper_allocator_api (main)
$ ltrace -x "malloc+free" target/debug/exper_allocator_api 2>&1 | rg free | wc -l
10
```

Full output:
```
wink@3900x 22-12-17T17:53:07.353Z:~/prgs/rust/myrepos/exper_allocator_api (main)
$ ltrace -x "malloc+free" target/debug/exper_allocator_api
malloc@libc.so.6(472)                                                                                        = 0x560b73b172a0
malloc@libc.so.6(120)                                                                                        = 0x560b73b17480
malloc@libc.so.6(1024)                                                                                       = 0x560b73b17500
free@libc.so.6(0x560b73b17910)                                                                               = <void>
free@libc.so.6(0x560b73b17500)                                                                               = <void>
free@libc.so.6(0x560b73b172a0)                                                                               = <void>
malloc@libc.so.6(32)                                                                                         = 0x560b73b17a10
malloc@libc.so.6(32)                                                                                         = 0x560b73b17ae0
free@libc.so.6(0x560b73b17a10)                                                                               = <void>
free@libc.so.6(0x560b73b17ae0)                                                                               = <void>
free@libc.so.6(0x560b73b17a40)                                                                               = <void>
malloc@libc.so.6(5)                                                                                          = 0x560b73b17b10
malloc@libc.so.6(48)                                                                                         = 0x560b73b17b30
malloc@libc.so.6(1024)                                                                                       = 0x560b73b17500
main:+
main inner:+
ma_init:+
malloc@libc.so.6(160)                                                                                        = 0x560b73b17ba0
malloc@libc.so.6(32)                                                                                         = 0x560b73b17ae0
ma_init:- len=10
MyAllocator::allocate: p_mut_u8=0x560b73b17c30
main inner: m=Add { left: 5, right: 6 }
MyAllocator::allocate: p_mut_u8=0x560b73b17c20
main inner: m=Add { left: 2, right: 4 }
main inner:-
MyAllocator::deallocate: p_mut_u8=0x560b73b17c20
MyAllocator::deallocate: p_mut_u8=0x560b73b17c30
main:-
free@libc.so.6(0x560b73b17500)                                                                               = <void>
free@libc.so.6(0x560b73b17b10)                                                                               = <void>
free@libc.so.6(0x560b73b17b30)                                                                               = <void>
free@libc.so.6(0x560b73b17b70)                                                                               = <void>
+++ exited (status 0) +++
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
