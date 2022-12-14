# Experiment with allocator_api

I want to test the hypothesis that a custom allocator
can manage a "Protocol" (an enum of message types) faster
that using the global allocator.

Currently compiles, runs and test passes, but valgrind fails :(

As suggested at, https://stackoverflow.com/a/56790193, I used
rust `alloc` & `delloc`. I'm going to try `libc` next.

## Running

```
wink@3900x 22-12-14T21:29:08.499Z:~/prgs/rust/myrepos/exper_allocator_api (main)
$ cargo run
   Compiling exper_allocator_api v0.1.0 (/home/wink/prgs/rust/myrepos/exper_allocator_api)
    Finished dev [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/exper_allocator_api`
main:+
main inner:+
MyAllocator::new:+
MyAllocator::new:- MyAllocator { data: 0x55835aa19ba0, layout: Layout { size: 160, align: 8 (1 << 3) }, count: 10, available: RefCell { value: [0x55835aa19ba0, 0x55835aa19c40, 0x55835aa19ce0, 0x55835aa19d80, 0x55835aa19e20, 0x55835aa19ec0, 0x55835aa19f60, 0x55835aa1a000, 0x55835aa1a0a0, 0x55835aa1a140] } }
allocate:+ layout align=8 size=16 self.available.len=10
allocate:  ptr=0x55835aa1a140 layout align=8 size=16
allocate:- ptr=0x55835aa1a140 layout align=8 size=16 self.available.len=9
main inner: m=Add { left: 5, right: 6 }
main inner:-
deallocate:+ ptr=0x55835aa1a140 layout align=8 size=16 self.available.len=9
deallocate: p_mut_u8=0x55835aa1a140
deallocate:- ptr=0x55835aa1a140 layout align=8 size=16 self.available.len=10
MyAllocator::drop:+ MyAllocator { data: 0x55835aa19ba0, layout: Layout { size: 160, align: 8 (1 << 3) }, count: 10, available: RefCell { value: [0x55835aa19ba0, 0x55835aa19c40, 0x55835aa19ce0, 0x55835aa19d80, 0x55835aa19e20, 0x55835aa19ec0, 0x55835aa19f60, 0x55835aa1a000, 0x55835aa1a0a0, 0x55835aa1a140] } }
MyAllocator::drop:-
main:-
```

## Tests

Passing
```
wink@3900x 22-12-14T21:29:10.217Z:~/prgs/rust/myrepos/exper_allocator_api (main)
$ cargo test
   Compiling exper_allocator_api v0.1.0 (/home/wink/prgs/rust/myrepos/exper_allocator_api)
    Finished test [unoptimized + debuginfo] target(s) in 0.15s
     Running unittests src/lib.rs (target/debug/deps/exper_allocator_api-caf5b2c13e23ac2e)

running 1 test
test tests::test_one_allocation ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/exper_allocator_api-8f6fcedfdba3680b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests exper_allocator_api

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Valgrind

`valgrind` fails:
 * ==57148== ERROR SUMMARY: 4 errors from 2 contexts (suppressed: 0 from 0)
```
wink@3900x 22-12-14T21:31:13.360Z:~/prgs/rust/myrepos/exper_allocator_api (main)
$ valgrind target/debug/exper_allocator_api 
==57148== Memcheck, a memory error detector
==57148== Copyright (C) 2002-2022, and GNU GPL'd, by Julian Seward et al.
==57148== Using Valgrind-3.19.0 and LibVEX; rerun with -h for copyright info
==57148== Command: target/debug/exper_allocator_api
==57148== 
main:+
main inner:+
MyAllocator::new:+
MyAllocator::new:- MyAllocator { data: 0x4a8bf90, layout: Layout { size: 160, align: 8 (1 << 3) }, count: 10, available: RefCell { value: [0x4a8bf90, 0x4a8c030, 0x4a8c0d0, 0x4a8c170, 0x4a8c210, 0x4a8c2b0, 0x4a8c350, 0x4a8c3f0, 0x4a8c490, 0x4a8c530] } }
allocate:+ layout align=8 size=16 self.available.len=10
allocate:  ptr=0x4a8c530 layout align=8 size=16
allocate:- ptr=0x4a8c530 layout align=8 size=16 self.available.len=9
==57148== Invalid write of size 8
==57148==    at 0x111285: write<exper_allocator_api::Protocol> (mod.rs:1332)
==57148==    by 0x111285: write<exper_allocator_api::Protocol> (mut_ptr.rs:1470)
==57148==    by 0x111285: alloc::boxed::Box<T,A>::new_in (boxed.rs:390)
==57148==    by 0x1108A1: exper_allocator_api::main (main.rs:20)
==57148==    by 0x110A2A: core::ops::function::FnOnce::call_once (function.rs:507)
==57148==    by 0x11191D: std::sys_common::backtrace::__rust_begin_short_backtrace (backtrace.rs:121)
==57148==    by 0x1115D0: std::rt::lang_start::{{closure}} (rt.rs:166)
==57148==    by 0x12A09B: call_once<(), (dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (function.rs:606)
==57148==    by 0x12A09B: do_call<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panicking.rs:483)
==57148==    by 0x12A09B: try<i32, &(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (panicking.rs:447)
==57148==    by 0x12A09B: catch_unwind<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panic.rs:137)
==57148==    by 0x12A09B: {closure#2} (rt.rs:148)
==57148==    by 0x12A09B: do_call<std::rt::lang_start_internal::{closure_env#2}, isize> (panicking.rs:483)
==57148==    by 0x12A09B: try<isize, std::rt::lang_start_internal::{closure_env#2}> (panicking.rs:447)
==57148==    by 0x12A09B: catch_unwind<std::rt::lang_start_internal::{closure_env#2}, isize> (panic.rs:137)
==57148==    by 0x12A09B: std::rt::lang_start_internal (rt.rs:148)
==57148==    by 0x1115A9: std::rt::lang_start (rt.rs:165)
==57148==    by 0x1109FD: main (in /home/wink/prgs/rust/myrepos/exper_allocator_api/target/debug/exper_allocator_api)
==57148==  Address 0x4a8c530 is 800 bytes inside an unallocated block of size 4,189,648 in arena "client"
==57148== 
==57148== Invalid read of size 8
==57148==    at 0x14C167: to_u64 (num.rs:45)
==57148==    by 0x14C167: core::fmt::num::imp::<impl core::fmt::Display for u64>::fmt (num.rs:282)
==57148==    by 0x114193: core::fmt::num::<impl core::fmt::Debug for u64>::fmt (num.rs:191)
==57148==    by 0x113D65: <&T as core::fmt::Debug>::fmt (mod.rs:2372)
==57148==    by 0x147A4D: {closure#0} (builders.rs:141)
==57148==    by 0x147A4D: and_then<(), core::fmt::Error, (), core::fmt::builders::{impl#3}::field::{closure_env#0}> (result.rs:1372)
==57148==    by 0x147A4D: core::fmt::builders::DebugStruct::field (builders.rs:124)
==57148==    by 0x148D13: core::fmt::Formatter::debug_struct_field2_finish (mod.rs:2007)
==57148==    by 0x112F5B: <exper_allocator_api::Protocol as core::fmt::Debug>::fmt (lib.rs:7)
==57148==    by 0x111517: <alloc::boxed::Box<T,A> as core::fmt::Debug>::fmt (boxed.rs:1884)
==57148==    by 0x14829D: core::fmt::write (mod.rs:1208)
==57148==    by 0x12B723: write_fmt<std::io::stdio::StdoutLock> (mod.rs:1682)
==57148==    by 0x12B723: <&std::io::stdio::Stdout as std::io::Write>::write_fmt (stdio.rs:716)
==57148==    by 0x12BE02: write_fmt (stdio.rs:690)
==57148==    by 0x12BE02: print_to<std::io::stdio::Stdout> (stdio.rs:1008)
==57148==    by 0x12BE02: std::io::stdio::_print (stdio.rs:1075)
==57148==    by 0x110933: exper_allocator_api::main (main.rs:23)
==57148==    by 0x110A2A: core::ops::function::FnOnce::call_once (function.rs:507)
==57148==  Address 0x4a8c530 is 800 bytes inside an unallocated block of size 4,189,648 in arena "client"
==57148== 
main inner: m=Add { left: 5, right: 6 }
main inner:-
deallocate:+ ptr=0x4a8c530 layout align=8 size=16 self.available.len=9
deallocate: p_mut_u8=0x4a8c530
deallocate:- ptr=0x4a8c530 layout align=8 size=16 self.available.len=10
MyAllocator::drop:+ MyAllocator { data: 0x4a8bf90, layout: Layout { size: 160, align: 8 (1 << 3) }, count: 10, available: RefCell { value: [0x4a8bf90, 0x4a8c030, 0x4a8c0d0, 0x4a8c170, 0x4a8c210, 0x4a8c2b0, 0x4a8c350, 0x4a8c3f0, 0x4a8c490, 0x4a8c530] } }
MyAllocator::drop:-
main:-
==57148== 
==57148== HEAP SUMMARY:
==57148==     in use at exit: 0 bytes in 0 blocks
==57148==   total heap usage: 15 allocs, 15 frees, 3,565 bytes allocated
==57148== 
==57148== All heap blocks were freed -- no leaks are possible
==57148== 
==57148== For lists of detected and suppressed errors, rerun with: -s
==57148== ERROR SUMMARY: 4 errors from 2 contexts (suppressed: 0 from 0)
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
