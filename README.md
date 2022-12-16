# Experiment with allocator_api

I want to test the hypothesis that a custom allocator
can manage a "Protocol" (an enum of message types) faster
that using the global allocator.

Currently compiles, runs and test passes, but valgrind and
sanitizer::AddressSanitizer both fail so I'm convinced I
have a Bug!

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

## ltrace

Using ltrace you can see the malloc and free calls, AFAICT there
is nothing wrong. `valgrind` is reporting that there are 8 bytes of
writes and reads outside of allocated space. The address is the
last 16 bytes of the allocated space. :
```
==57148==  Address 0x4a8c530 is 800 bytes inside an unallocated block of size 4,189,648 in arena "client"
```

But using ltrace, while the addresses are different from `valgrind`
data, you can see the order is "correct".
 1) line 17 the 160 bytes is allocated from the system using `malloc`
 2) line 22 we've allocated by pop from `available`
 3) line 24 we've deallocated by pushing to `available`
 4) line 27 it its returned to the system using `free`

 So everything LGTM, and I have no idea why `valgrind` is complaining :(
```
    17  malloc@libc.so.6(160)                            = 0x557258d25ba0
    18  malloc@libc.so.6(32)                             = 0x557258d25ae0
    19  MyAllocator::new:- MyAllocator { data: 0x557258d25ba0, layout: Layout { size: 160, align: 8 (1 << 3) }, count: 10, available: RefCell { value: [0x557258d25ba0, 0x557258d25c40, 0x557258d25ce0, 0x557258d25d80, 0x557258d25e20, 0x557258d25ec0, 0x557258d25f60, 0x557258d26000, 0x557258d260a0, 0x557258d26140] } }
    20  allocate:+ layout align=8 size=16 self.available.len=10
    21  allocate:  ptr=0x557258d26140 layout align=8 size=16
    22  allocate:- ptr=0x557258d26140 layout align=8 size=16 self.available.len=9
    23  deallocate:+ ptr=0x557258d26140 layout align=8 size=16 self.available.len=9
    24  deallocate: p_mut_u8=0x557258d26140
    25  deallocate:- ptr=0x557258d26140 layout align=8 size=16 self.available.len=10
    26  MyAllocator::drop:+ MyAllocator { data: 0x557258d25ba0, layout: Layout { size: 160, align: 8 (1 << 3) }, count: 10, available: RefCell { value: [0x557258d25ba0, 0x557258d25c40, 0x557258d25ce0, 0x557258d25d80, 0x557258d25e20, 0x557258d25ec0, 0x557258d25f60, 0x557258d26000, 0x557258d260a0, 0x557258d26140] } }
    27  free@libc.so.6(0x557258d25ba0)                   = <void>
    28  MyAllocator::drop:-
```

Here is the full `ltrace` output:
```
wink@3900x 22-12-15T03:54:36.260Z:~/prgs/rust/myrepos/exper_allocator_api (main)
$ ltrace -x "malloc+free*" target/debug/exper_allocator_api > ltrace.txt 2>&1
wink@3900x 22-12-15T03:55:06.652Z:~/prgs/rust/myrepos/exper_allocator_api (main)
$ cat -n ltrace.txt 
     1  malloc@libc.so.6(472)                            = 0x557258d252a0
     2  malloc@libc.so.6(120)                            = 0x557258d25480
     3  malloc@libc.so.6(1024)                           = 0x557258d25500
     4  free@libc.so.6(0x557258d25910)                   = <void>
     5  free@libc.so.6(0x557258d25500)                   = <void>
     6  free@libc.so.6(0x557258d252a0)                   = <void>
     7  malloc@libc.so.6(32)                             = 0x557258d25a10
     8  malloc@libc.so.6(32)                             = 0x557258d25ae0
     9  free@libc.so.6(0x557258d25a10)                   = <void>
    10  free@libc.so.6(0x557258d25ae0)                   = <void>
    11  free@libc.so.6(0x557258d25a40)                   = <void>
    12  malloc@libc.so.6(5)                              = 0x557258d25b10
    13  malloc@libc.so.6(48)                             = 0x557258d25b30
    14  malloc@libc.so.6(1024)                           = 0x557258d25500
    15  main:+
    16  MyAllocator::new:+
    17  malloc@libc.so.6(160)                            = 0x557258d25ba0
    18  malloc@libc.so.6(32)                             = 0x557258d25ae0
    19  MyAllocator::new:- MyAllocator { data: 0x557258d25ba0, layout: Layout { size: 160, align: 8 (1 << 3) }, count: 10, available: RefCell { value: [0x557258d25ba0, 0x557258d25c40, 0x557258d25ce0, 0x557258d25d80, 0x557258d25e20, 0x557258d25ec0, 0x557258d25f60, 0x557258d26000, 0x557258d260a0, 0x557258d26140] } }
    20  allocate:+ layout align=8 size=16 self.available.len=10
    21  allocate:  ptr=0x557258d26140 layout align=8 size=16
    22  allocate:- ptr=0x557258d26140 layout align=8 size=16 self.available.len=9
    23  deallocate:+ ptr=0x557258d26140 layout align=8 size=16 self.available.len=9
    24  deallocate: p_mut_u8=0x557258d26140
    25  deallocate:- ptr=0x557258d26140 layout align=8 size=16 self.available.len=10
    26  MyAllocator::drop:+ MyAllocator { data: 0x557258d25ba0, layout: Layout { size: 160, align: 8 (1 << 3) }, count: 10, available: RefCell { value: [0x557258d25ba0, 0x557258d25c40, 0x557258d25ce0, 0x557258d25d80, 0x557258d25e20, 0x557258d25ec0, 0x557258d25f60, 0x557258d26000, 0x557258d260a0, 0x557258d26140] } }
    27  free@libc.so.6(0x557258d25ba0)                   = <void>
    28  MyAllocator::drop:-
    29  free@libc.so.6(0x557258d25c50)                   = <void>
    30  main:-
    31  free@libc.so.6(0x557258d25500)                   = <void>
    32  free@libc.so.6(0x557258d25b10)                   = <void>
    33  free@libc.so.6(0x557258d25b30)                   = <void>
    34  free@libc.so.6(0x557258d25b70)                   = <void>
    35  +++ exited (status 0) +++
```

## stanitizer AddressSanitizer

AddressStanitizer has provided "better" information, it
provides the stack backtrace of where the bad write happened.

```
wink@3900x 22-12-16T16:24:24.101Z:~/prgs/rust/myrepos/exper_allocator_api (try-sanitizer)
$ RUSTFLAGS=-Zsanitizer=address cargo run -Zbuild-std --target x86_64-unknown-linux-gnu > asan-output.txt 2>&1
wink@3900x 22-12-16T16:31:00.874Z:~/prgs/rust/myrepos/exper_allocator_api (try-sanitizer)
$ cat -n asan-output.txt 
     1      Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     2       Running `target/x86_64-unknown-linux-gnu/debug/exper_allocator_api`
     3  main:+
     4  main inner:+
     5  MyAllocator::new:+
     6  MyAllocator::new:- MyAllocator { data: 0x60e000000120, layout: Layout { size: 160, align: 8 (1 << 3) }, count: 10, available: RefCell { value: [0x60e000000120, 0x60e0000001c0, 0x60e000000260, 0x60e000000300, 0x60e0000003a0, 0x60e000000440, 0x60e0000004e0, 0x60e000000580, 0x60e000000620, 0x60e0000006c0] } }
     7  allocate:+ layout align=8 size=16 self.available.len=10
     8  allocate:  ptr=0x60e0000006c0 layout align=8 size=16
     9  allocate:- ptr=0x60e0000006c0 layout align=8 size=16 self.available.len=9
    10  =================================================================
    11  ==17424==ERROR: AddressSanitizer: heap-buffer-overflow on address 0x60e0000006c0 at pc 0x55b0b84a926a bp 0x7ffe8bae4810 sp 0x7ffe8bae3fe0
    12  WRITE of size 16 at 0x60e0000006c0 thread T0
    13      #0 0x55b0b84a9269 in __asan_memcpy /rustc/llvm/src/llvm-project/compiler-rt/lib/asan/asan_interceptors_memintrinsics.cpp:22:3
    14      #1 0x55b0b84d6cb9 in core::ptr::write::h23c78b6a3ead4e49 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:1332:9
    15      #2 0x55b0b84d58d5 in core::ptr::mut_ptr::_$LT$impl$u20$$BP$mut$u20$T$GT$::write::h5480f407fd9ca7f9 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:1470:18
    16      #3 0x55b0b84d58d5 in alloc::boxed::Box$LT$T$C$A$GT$::new_in::h913decf6f47fe053 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:390:13
    17      #4 0x55b0b84d174f in exper_allocator_api::main::h5ebd3fbe2831e9e8 /home/wink/prgs/rust/myrepos/exper_allocator_api/src/main.rs:20:17
    18      #5 0x55b0b84d6ffa in core::ops::function::FnOnce::call_once::h857f9c6d14e37c0c /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:507:5
    19      #6 0x55b0b84d2257 in std::sys_common::backtrace::__rust_begin_short_backtrace::h1ee23575c015f0c0 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys_common/backtrace.rs:121:18
    20      #7 0x55b0b84d7473 in std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h0dbe6260aed6e062 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:166:18
    21      #8 0x55b0b8821f6d in core::ops::function::impls::_$LT$impl$u20$core..ops..function..FnOnce$LT$A$GT$$u20$for$u20$$RF$F$GT$::call_once::h750779b9c5158193 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:606:13
    22      #9 0x55b0b85d81be in std::panicking::try::do_call::h41b4d73441e5a27f /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:483:40
    23      #10 0x55b0b85dbd4a in __rust_try std.63ac60a6-cgu.35
    24      #11 0x55b0b85d6de5 in std::panicking::try::h2155994ae210243d /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:447:19
    25      #12 0x55b0b84eebc9 in std::panic::catch_unwind::hc7326399cad39c91 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:137:14
    26      #13 0x55b0b84e88c0 in std::rt::lang_start_internal::_$u7b$$u7b$closure$u7d$$u7d$::h0fa04e3694c250f7 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:148:48
    27      #14 0x55b0b85d8396 in std::panicking::try::do_call::h928ac4600abff174 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:483:40
    28      #15 0x55b0b85dbd4a in __rust_try std.63ac60a6-cgu.35
    29      #16 0x55b0b85d73d5 in std::panicking::try::hd2b4bb59e4bb40ad /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:447:19
    30      #17 0x55b0b84eea59 in std::panic::catch_unwind::h8883ea9e4dca3787 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:137:14
    31      #18 0x55b0b84e82f5 in std::rt::lang_start_internal::hceb5e973a747532f /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:148:20
    32      #19 0x55b0b84d73cf in std::rt::lang_start::h50287bcd24139cd6 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:165:17
    33      #20 0x55b0b84d1b2d in main (/home/wink/prgs/rust/myrepos/exper_allocator_api/target/x86_64-unknown-linux-gnu/debug/exper_allocator_api+0x195b2d) (BuildId: 2ac285698e4e7b28df9782cdca3bd72ad0b6fb23)
    34      #21 0x7f803fab028f  (/usr/lib/libc.so.6+0x2328f) (BuildId: 1e94beb079e278ac4f2c8bce1f53091548ea1584)
    35      #22 0x7f803fab0349 in __libc_start_main (/usr/lib/libc.so.6+0x23349) (BuildId: 1e94beb079e278ac4f2c8bce1f53091548ea1584)
    36      #23 0x55b0b8435254 in _start /build/glibc/src/glibc/csu/../sysdeps/x86_64/start.S:115
    37
    38  0x60e0000006c0 is located 1280 bytes to the right of 160-byte region [0x60e000000120,0x60e0000001c0)
    39  allocated by thread T0 here:
    40      #0 0x55b0b84a9cbe in malloc /rustc/llvm/src/llvm-project/compiler-rt/lib/asan/asan_malloc_linux.cpp:69:3
    41      #1 0x55b0b84eb426 in std::sys::unix::alloc::_$LT$impl$u20$core..alloc..global..GlobalAlloc$u20$for$u20$std..alloc..System$GT$::alloc::hf2cbfb265a23c366 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/unix/alloc.rs:14:13
    42      #2 0x55b0b854673f in __rdl_alloc /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/alloc.rs:381:13
    43      #3 0x55b0b84e2c26 in alloc::alloc::alloc::hcf7ebaf9b5126d6c /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:95:14
    44      #4 0x55b0b84d78af in exper_allocator_api::MyAllocator::new::hbba60a6ab62326c2 /home/wink/prgs/rust/myrepos/exper_allocator_api/src/lib.rs:26:29
    45      #5 0x55b0b84d1634 in exper_allocator_api::main::h5ebd3fbe2831e9e8 /home/wink/prgs/rust/myrepos/exper_allocator_api/src/main.rs:17:18
    46      #6 0x55b0b84d6ffa in core::ops::function::FnOnce::call_once::h857f9c6d14e37c0c /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:507:5
    47      #7 0x55b0b84d7473 in std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h0dbe6260aed6e062 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:166:18
    48      #8 0x55b0b85d81be in std::panicking::try::do_call::h41b4d73441e5a27f /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:483:40
    49      #9 0x55b0b85dbd4a in __rust_try std.63ac60a6-cgu.35
    50      #10 0x55b0b84eebc9 in std::panic::catch_unwind::hc7326399cad39c91 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:137:14
    51      #11 0x55b0b84e88c0 in std::rt::lang_start_internal::_$u7b$$u7b$closure$u7d$$u7d$::h0fa04e3694c250f7 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:148:48
    52      #12 0x55b0b85d8396 in std::panicking::try::do_call::h928ac4600abff174 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:483:40
    53      #13 0x55b0b85dbd4a in __rust_try std.63ac60a6-cgu.35
    54      #14 0x55b0b84eea59 in std::panic::catch_unwind::h8883ea9e4dca3787 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:137:14
    55      #15 0x55b0b84e82f5 in std::rt::lang_start_internal::hceb5e973a747532f /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:148:20
    56      #16 0x55b0b84d73cf in std::rt::lang_start::h50287bcd24139cd6 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:165:17
    57      #17 0x55b0b84d1b2d in main (/home/wink/prgs/rust/myrepos/exper_allocator_api/target/x86_64-unknown-linux-gnu/debug/exper_allocator_api+0x195b2d) (BuildId: 2ac285698e4e7b28df9782cdca3bd72ad0b6fb23)
    58
    59  SUMMARY: AddressSanitizer: heap-buffer-overflow /rustc/llvm/src/llvm-project/compiler-rt/lib/asan/asan_interceptors_memintrinsics.cpp:22:3 in __asan_memcpy
    60  Shadow bytes around the buggy address:
    61    0x0c1c7fff8080: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    62    0x0c1c7fff8090: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    63    0x0c1c7fff80a0: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    64    0x0c1c7fff80b0: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    65    0x0c1c7fff80c0: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    66  =>0x0c1c7fff80d0: fa fa fa fa fa fa fa fa[fa]fa fa fa fa fa fa fa
    67    0x0c1c7fff80e0: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    68    0x0c1c7fff80f0: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    69    0x0c1c7fff8100: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    70    0x0c1c7fff8110: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    71    0x0c1c7fff8120: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    72  Shadow byte legend (one shadow byte represents 8 application bytes):
    73    Addressable:           00
    74    Partially addressable: 01 02 03 04 05 06 07 
    75    Heap left redzone:       fa
    76    Freed heap region:       fd
    77    Stack left redzone:      f1
    78    Stack mid redzone:       f2
    79    Stack right redzone:     f3
    80    Stack after return:      f5
    81    Stack use after scope:   f8
    82    Global redzone:          f9
    83    Global init order:       f6
    84    Poisoned by user:        f7
    85    Container overflow:      fc
    86    Array cookie:            ac
    87    Intra object redzone:    bb
    88    ASan internal:           fe
    89    Left alloca redzone:     ca
    90    Right alloca redzone:    cb
    91  ==17424==ABORTING
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
