# Experiment with allocator_api

I want to test the hypothesis that a custom allocator
can manage a "Protocol" (an enum of message types) faster
that using the global allocator.

Currently compiles, runs and test passes, but valgrind and
sanitizer::AddressSanitizer both fail so I'm convinced I
have a Bug!

## Running

```
wink@3900x 22-12-16T18:28:41.055Z:~/prgs/rust/myrepos/exper_allocator_api (main)
$ cargo run
   Compiling exper_allocator_api v0.2.0 (/home/wink/prgs/rust/myrepos/exper_allocator_api)
    Finished dev [unoptimized + debuginfo] target(s) in 0.39s
     Running `target/debug/exper_allocator_api`
main:+
main inner:+
MyAllocator::new:+
MyAllocator::new:- MyAllocator { data: 0x55a899fa7ba0, layout: Layout { size: 160, align: 8 (1 << 3) }, count: 10, available: RefCell { value: [0x55a899fa7ba0, 0x55a899fa7bb0, 0x55a899fa7bc0, 0x55a899fa7bd0, 0x55a899fa7be0, 0x55a899fa7bf0, 0x55a899fa7c00, 0x55a899fa7c10, 0x55a899fa7c20, 0x55a899fa7c30] } }
allocate:+ layout align=8 size=16 self.available.len=10
allocate:  ptr=0x55a899fa7c30 layout align=8 size=16
allocate:- ptr=0x55a899fa7c30 layout align=8 size=16 self.available.len=9
main inner: m=Add { left: 5, right: 6 }
main inner:-
deallocate:+ ptr=0x55a899fa7c30 layout align=8 size=16 self.available.len=9
deallocate: p_mut_u8=0x55a899fa7c30
deallocate:- ptr=0x55a899fa7c30 layout align=8 size=16 self.available.len=10
MyAllocator::drop:+ MyAllocator { data: 0x55a899fa7ba0, layout: Layout { size: 160, align: 8 (1 << 3) }, count: 10, available: RefCell { value: [0x55a899fa7ba0, 0x55a899fa7bb0, 0x55a899fa7bc0, 0x55a899fa7bd0, 0x55a899fa7be0, 0x55a899fa7bf0, 0x55a899fa7c00, 0x55a899fa7c10, 0x55a899fa7c20, 0x55a899fa7c30] } }
MyAllocator::drop:-
main:-
```

## Tests

Passing
```
wink@3900x 22-12-16T18:28:43.414Z:~/prgs/rust/myrepos/exper_allocator_api (main)
$ cargo test
   Compiling exper_allocator_api v0.2.0 (/home/wink/prgs/rust/myrepos/exper_allocator_api)
    Finished test [unoptimized + debuginfo] target(s) in 0.23s
     Running unittests src/lib.rs (target/debug/deps/exper_allocator_api-32c159fd1fc5d893)

running 1 test
test tests::test_one_allocation ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/exper_allocator_api-095f7b6c7e6c5816)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests exper_allocator_api

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Valgrind

`valgrind` succeeds:
```
wink@3900x 22-12-16T18:29:04.741Z:~/prgs/rust/myrepos/exper_allocator_api (main)
$ valgrind target/debug/exper_allocator_api
==23949== Memcheck, a memory error detector
==23949== Copyright (C) 2002-2022, and GNU GPL'd, by Julian Seward et al.
==23949== Using Valgrind-3.19.0 and LibVEX; rerun with -h for copyright info
==23949== Command: target/debug/exper_allocator_api
==23949== 
main:+
main inner:+
MyAllocator::new:+
MyAllocator::new:- MyAllocator { data: 0x4a8bf90, layout: Layout { size: 160, align: 8 (1 << 3) }, count: 10, available: RefCell { value: [0x4a8bf90, 0x4a8bfa0, 0x4a8bfb0, 0x4a8bfc0, 0x4a8bfd0, 0x4a8bfe0, 0x4a8bff0, 0x4a8c000, 0x4a8c010, 0x4a8c020] } }
allocate:+ layout align=8 size=16 self.available.len=10
allocate:  ptr=0x4a8c020 layout align=8 size=16
allocate:- ptr=0x4a8c020 layout align=8 size=16 self.available.len=9
main inner: m=Add { left: 5, right: 6 }
main inner:-
deallocate:+ ptr=0x4a8c020 layout align=8 size=16 self.available.len=9
deallocate: p_mut_u8=0x4a8c020
deallocate:- ptr=0x4a8c020 layout align=8 size=16 self.available.len=10
MyAllocator::drop:+ MyAllocator { data: 0x4a8bf90, layout: Layout { size: 160, align: 8 (1 << 3) }, count: 10, available: RefCell { value: [0x4a8bf90, 0x4a8bfa0, 0x4a8bfb0, 0x4a8bfc0, 0x4a8bfd0, 0x4a8bfe0, 0x4a8bff0, 0x4a8c000, 0x4a8c010, 0x4a8c020] } }
MyAllocator::drop:-
main:-
==23949== 
==23949== HEAP SUMMARY:
==23949==     in use at exit: 0 bytes in 0 blocks
==23949==   total heap usage: 15 allocs, 15 frees, 3,565 bytes allocated
==23949== 
==23949== All heap blocks were freed -- no leaks are possible
==23949== 
==23949== For lists of detected and suppressed errors, rerun with: -s
==23949== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
```

## stanitizer AddressSanitizer

AddressSanitizer succeeds:
```
wink@3900x 22-12-16T18:30:11.976Z:~/prgs/rust/myrepos/exper_allocator_api (main)
$ RUSTFLAGS=-Zsanitizer=address cargo run -Zbuild-std --target x86_64-unknown-linux-gnu
   Compiling compiler_builtins v0.1.85
   Compiling core v0.0.0 (/home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core)
   Compiling libc v0.2.138
   Compiling cc v1.0.76
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std)
   Compiling unwind v0.0.0 (/home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/unwind)
   Compiling rustc-std-workspace-core v1.99.0 (/home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/rustc-std-workspace-core)
   Compiling alloc v0.0.0 (/home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc)
   Compiling cfg-if v1.0.0
   Compiling adler v1.0.2
   Compiling rustc-demangle v0.1.21
   Compiling rustc-std-workspace-alloc v1.99.0 (/home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/rustc-std-workspace-alloc)
   Compiling panic_abort v0.0.0 (/home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/panic_abort)
   Compiling panic_unwind v0.0.0 (/home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/panic_unwind)
   Compiling gimli v0.26.1
   Compiling std_detect v0.1.5 (/home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/stdarch/crates/std_detect)
   Compiling object v0.29.0
   Compiling miniz_oxide v0.5.3
   Compiling hashbrown v0.12.3
   Compiling addr2line v0.17.0
   Compiling proc_macro v0.0.0 (/home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/proc_macro)
   Compiling exper_allocator_api v0.2.0 (/home/wink/prgs/rust/myrepos/exper_allocator_api)
    Finished dev [unoptimized + debuginfo] target(s) in 11.97s
     Running `target/x86_64-unknown-linux-gnu/debug/exper_allocator_api`
main:+
main inner:+
MyAllocator::new:+
MyAllocator::new:- MyAllocator { data: 0x60e000000120, layout: Layout { size: 160, align: 8 (1 << 3) }, count: 10, available: RefCell { value: [0x60e000000120, 0x60e000000130, 0x60e000000140, 0x60e000000150, 0x60e000000160, 0x60e000000170, 0x60e000000180, 0x60e000000190, 0x60e0000001a0, 0x60e0000001b0] } }
allocate:+ layout align=8 size=16 self.available.len=10
allocate:  ptr=0x60e0000001b0 layout align=8 size=16
allocate:- ptr=0x60e0000001b0 layout align=8 size=16 self.available.len=9
main inner: m=Add { left: 5, right: 6 }
main inner:-
deallocate:+ ptr=0x60e0000001b0 layout align=8 size=16 self.available.len=9
deallocate: p_mut_u8=0x60e0000001b0
deallocate:- ptr=0x60e0000001b0 layout align=8 size=16 self.available.len=10
MyAllocator::drop:+ MyAllocator { data: 0x60e000000120, layout: Layout { size: 160, align: 8 (1 << 3) }, count: 10, available: RefCell { value: [0x60e000000120, 0x60e000000130, 0x60e000000140, 0x60e000000150, 0x60e000000160, 0x60e000000170, 0x60e000000180, 0x60e000000190, 0x60e0000001a0, 0x60e0000001b0] } }
MyAllocator::drop:-
main:-
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
