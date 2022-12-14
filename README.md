# Experiment with allocator_api

I want to test the hypothesis that a custom allocator
can manage a "Protocol" (an enum of message types) faster
that using the global allocator.

Currently it compiles and runs:

```
wink@3900x 22-12-14T19:08:29.053Z:~/prgs/rust/myrepos/exper_allocator_api (main)
$ cargo run
   Compiling exper_allocator_api v0.1.0 (/home/wink/prgs/rust/myrepos/exper_allocator_api)
    Finished dev [unoptimized + debuginfo] target(s) in 0.39s
     Running `target/debug/exper_allocator_api`
main:+
main inner:+
main inner: 1
main inner: 2
allocate:+ layout align=8 size=16
allocate: Got one from avaiable
allocate: thing=Add { left: 0, right: 0 }
allocate: 0x55d00c9ccba0 [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]
allocate:- ptr=0x55d00c9ccba0 layout align=8 size=16
main inner: 3
main inner: m=Add { left: 5, right: 6 } &*m=0x55d00c9ccba0
main inner: bp=Add { left: 5, right: 6 }
main inner:-
deallocate:+ ptr=0x55d00c9ccba0 layout align=8 size=16
deallocate: 0x55d00c9ccba0 [05, 00, 00, 00, 00, 00, 00, 00, 06, 00, 00, 00, 00, 00, 00, 00]
deallocate:  1
deallocate:  2
deallocate:- ptr=0x55d00c9ccba0 layout align=8 size=16
main:-
```

But the test fails:

```
wink@3900x 22-12-14T19:08:32.382Z:~/prgs/rust/myrepos/exper_allocator_api (main)
$ cargo test
   Compiling exper_allocator_api v0.1.0 (/home/wink/prgs/rust/myrepos/exper_allocator_api)
    Finished test [unoptimized + debuginfo] target(s) in 0.22s
     Running unittests src/lib.rs (target/debug/deps/exper_allocator_api-caf5b2c13e23ac2e)

running 1 test
double free or corruption (fasttop)
error: test failed, to rerun pass `--lib`

Caused by:
  process didn't exit successfully: `/home/wink/prgs/rust/myrepos/exper_allocator_api/target/debug/deps/exper_allocator_api-caf5b2c13e23ac2e` (signal: 6, SIGABRT: process abort signal)
```

And running under `valgrind` says:
 * ==41800== ERROR SUMMARY: 23 errors from 6 contexts (suppressed: 0 from 0)
```
wink@3900x 22-12-14T19:08:57.175Z:~/prgs/rust/myrepos/exper_allocator_api (main)
$ valgrind target/debug/exper_allocator_api 
==41800== Memcheck, a memory error detector
==41800== Copyright (C) 2002-2022, and GNU GPL'd, by Julian Seward et al.
==41800== Using Valgrind-3.19.0 and LibVEX; rerun with -h for copyright info
==41800== Command: target/debug/exper_allocator_api
==41800== 
main:+
main inner:+
main inner: 1
main inner: 2
allocate:+ layout align=8 size=16
allocate: Got one from avaiable
allocate: thing=Add { left: 0, right: 0 }
allocate: 0x4a8bf90 [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]
allocate:- ptr=0x4a8bf90 layout align=8 size=16
==41800== Invalid write of size 8
==41800==    at 0x11157A: write<exper_allocator_api::Protocol> (mod.rs:1332)
==41800==    by 0x11157A: write<exper_allocator_api::Protocol> (mut_ptr.rs:1470)
==41800==    by 0x11157A: alloc::boxed::Box<T,A>::new_in (boxed.rs:390)
==41800==    by 0x110A34: exper_allocator_api::main (main.rs:38)
==41800==    by 0x110D3A: core::ops::function::FnOnce::call_once (function.rs:507)
==41800==    by 0x111C9D: std::sys_common::backtrace::__rust_begin_short_backtrace (backtrace.rs:121)
==41800==    by 0x111950: std::rt::lang_start::{{closure}} (rt.rs:166)
==41800==    by 0x129FEB: call_once<(), (dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (function.rs:606)
==41800==    by 0x129FEB: do_call<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panicking.rs:483)
==41800==    by 0x129FEB: try<i32, &(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (panicking.rs:447)
==41800==    by 0x129FEB: catch_unwind<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panic.rs:137)
==41800==    by 0x129FEB: {closure#2} (rt.rs:148)
==41800==    by 0x129FEB: do_call<std::rt::lang_start_internal::{closure_env#2}, isize> (panicking.rs:483)
==41800==    by 0x129FEB: try<isize, std::rt::lang_start_internal::{closure_env#2}> (panicking.rs:447)
==41800==    by 0x129FEB: catch_unwind<std::rt::lang_start_internal::{closure_env#2}, isize> (panic.rs:137)
==41800==    by 0x129FEB: std::rt::lang_start_internal (rt.rs:148)
==41800==    by 0x111929: std::rt::lang_start (rt.rs:165)
==41800==    by 0x110D0D: main (in /home/wink/prgs/rust/myrepos/exper_allocator_api/target/debug/exper_allocator_api)
==41800==  Address 0x4a8bf90 is 0 bytes inside a block of size 16 free'd
==41800==    at 0x484426F: free (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==41800==    by 0x115206: dealloc (alloc.rs:113)
==41800==    by 0x115206: <alloc::alloc::Global as core::alloc::Allocator>::deallocate (alloc.rs:250)
==41800==    by 0x114551: alloc::alloc::box_free (alloc.rs:348)
==41800==    by 0x1138DB: core::ptr::drop_in_place<alloc::boxed::Box<exper_allocator_api::Protocol>> (mod.rs:490)
==41800==    by 0x112C4E: <exper_allocator_api::MyAllocator as core::alloc::Allocator>::allocate (lib.rs:64)
==41800==    by 0x1111FE: alloc::boxed::Box<T,A>::try_new_uninit_in (boxed.rs:493)
==41800==    by 0x1110C2: alloc::boxed::Box<T,A>::new_uninit_in (boxed.rs:457)
==41800==    by 0x1114F2: alloc::boxed::Box<T,A>::new_in (boxed.rs:388)
==41800==    by 0x110A34: exper_allocator_api::main (main.rs:38)
==41800==    by 0x110D3A: core::ops::function::FnOnce::call_once (function.rs:507)
==41800==    by 0x111C9D: std::sys_common::backtrace::__rust_begin_short_backtrace (backtrace.rs:121)
==41800==    by 0x111950: std::rt::lang_start::{{closure}} (rt.rs:166)
==41800==  Block was alloc'd at
==41800==    at 0x4841888: malloc (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==41800==    by 0x1149AB: alloc::alloc::Global::alloc_impl (alloc.rs:95)
==41800==    by 0x114758: allocate (alloc.rs:237)
==41800==    by 0x114758: alloc::alloc::exchange_malloc (alloc.rs:326)
==41800==    by 0x111EA2: exper_allocator_api::MyAllocator::new (boxed.rs:220)
==41800==    by 0x110934: exper_allocator_api::main (main.rs:34)
==41800==    by 0x110D3A: core::ops::function::FnOnce::call_once (function.rs:507)
==41800==    by 0x111C9D: std::sys_common::backtrace::__rust_begin_short_backtrace (backtrace.rs:121)
==41800==    by 0x111950: std::rt::lang_start::{{closure}} (rt.rs:166)
==41800==    by 0x129FEB: call_once<(), (dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (function.rs:606)
==41800==    by 0x129FEB: do_call<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panicking.rs:483)
==41800==    by 0x129FEB: try<i32, &(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (panicking.rs:447)
==41800==    by 0x129FEB: catch_unwind<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panic.rs:137)
==41800==    by 0x129FEB: {closure#2} (rt.rs:148)
==41800==    by 0x129FEB: do_call<std::rt::lang_start_internal::{closure_env#2}, isize> (panicking.rs:483)
==41800==    by 0x129FEB: try<isize, std::rt::lang_start_internal::{closure_env#2}> (panicking.rs:447)
==41800==    by 0x129FEB: catch_unwind<std::rt::lang_start_internal::{closure_env#2}, isize> (panic.rs:137)
==41800==    by 0x129FEB: std::rt::lang_start_internal (rt.rs:148)
==41800==    by 0x111929: std::rt::lang_start (rt.rs:165)
==41800==    by 0x110D0D: main (in /home/wink/prgs/rust/myrepos/exper_allocator_api/target/debug/exper_allocator_api)
==41800== 
main inner: 3
==41800== Invalid read of size 8
==41800==    at 0x14BD27: to_u64 (num.rs:45)
==41800==    by 0x14BD27: core::fmt::num::imp::<impl core::fmt::Display for u64>::fmt (num.rs:282)
==41800==    by 0x114353: core::fmt::num::<impl core::fmt::Debug for u64>::fmt (num.rs:191)
==41800==    by 0x113E45: <&T as core::fmt::Debug>::fmt (mod.rs:2372)
==41800==    by 0x14797D: {closure#0} (builders.rs:141)
==41800==    by 0x14797D: and_then<(), core::fmt::Error, (), core::fmt::builders::{impl#3}::field::{closure_env#0}> (result.rs:1372)
==41800==    by 0x14797D: core::fmt::builders::DebugStruct::field (builders.rs:124)
==41800==    by 0x148C43: core::fmt::Formatter::debug_struct_field2_finish (mod.rs:2007)
==41800==    by 0x11332B: <exper_allocator_api::Protocol as core::fmt::Debug>::fmt (lib.rs:4)
==41800==    by 0x111897: <alloc::boxed::Box<T,A> as core::fmt::Debug>::fmt (boxed.rs:1884)
==41800==    by 0x1481CD: core::fmt::write (mod.rs:1208)
==41800==    by 0x12B673: write_fmt<std::io::stdio::StdoutLock> (mod.rs:1682)
==41800==    by 0x12B673: <&std::io::stdio::Stdout as std::io::Write>::write_fmt (stdio.rs:716)
==41800==    by 0x12BD52: write_fmt (stdio.rs:690)
==41800==    by 0x12BD52: print_to<std::io::stdio::Stdout> (stdio.rs:1008)
==41800==    by 0x12BD52: std::io::stdio::_print (stdio.rs:1075)
==41800==    by 0x110BA4: exper_allocator_api::main (main.rs:40)
==41800==    by 0x110D3A: core::ops::function::FnOnce::call_once (function.rs:507)
==41800==  Address 0x4a8bf90 is 0 bytes inside a block of size 16 free'd
==41800==    at 0x484426F: free (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==41800==    by 0x115206: dealloc (alloc.rs:113)
==41800==    by 0x115206: <alloc::alloc::Global as core::alloc::Allocator>::deallocate (alloc.rs:250)
==41800==    by 0x114551: alloc::alloc::box_free (alloc.rs:348)
==41800==    by 0x1138DB: core::ptr::drop_in_place<alloc::boxed::Box<exper_allocator_api::Protocol>> (mod.rs:490)
==41800==    by 0x112C4E: <exper_allocator_api::MyAllocator as core::alloc::Allocator>::allocate (lib.rs:64)
==41800==    by 0x1111FE: alloc::boxed::Box<T,A>::try_new_uninit_in (boxed.rs:493)
==41800==    by 0x1110C2: alloc::boxed::Box<T,A>::new_uninit_in (boxed.rs:457)
==41800==    by 0x1114F2: alloc::boxed::Box<T,A>::new_in (boxed.rs:388)
==41800==    by 0x110A34: exper_allocator_api::main (main.rs:38)
==41800==    by 0x110D3A: core::ops::function::FnOnce::call_once (function.rs:507)
==41800==    by 0x111C9D: std::sys_common::backtrace::__rust_begin_short_backtrace (backtrace.rs:121)
==41800==    by 0x111950: std::rt::lang_start::{{closure}} (rt.rs:166)
==41800==  Block was alloc'd at
==41800==    at 0x4841888: malloc (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==41800==    by 0x1149AB: alloc::alloc::Global::alloc_impl (alloc.rs:95)
==41800==    by 0x114758: allocate (alloc.rs:237)
==41800==    by 0x114758: alloc::alloc::exchange_malloc (alloc.rs:326)
==41800==    by 0x111EA2: exper_allocator_api::MyAllocator::new (boxed.rs:220)
==41800==    by 0x110934: exper_allocator_api::main (main.rs:34)
==41800==    by 0x110D3A: core::ops::function::FnOnce::call_once (function.rs:507)
==41800==    by 0x111C9D: std::sys_common::backtrace::__rust_begin_short_backtrace (backtrace.rs:121)
==41800==    by 0x111950: std::rt::lang_start::{{closure}} (rt.rs:166)
==41800==    by 0x129FEB: call_once<(), (dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (function.rs:606)
==41800==    by 0x129FEB: do_call<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panicking.rs:483)
==41800==    by 0x129FEB: try<i32, &(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (panicking.rs:447)
==41800==    by 0x129FEB: catch_unwind<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panic.rs:137)
==41800==    by 0x129FEB: {closure#2} (rt.rs:148)
==41800==    by 0x129FEB: do_call<std::rt::lang_start_internal::{closure_env#2}, isize> (panicking.rs:483)
==41800==    by 0x129FEB: try<isize, std::rt::lang_start_internal::{closure_env#2}> (panicking.rs:447)
==41800==    by 0x129FEB: catch_unwind<std::rt::lang_start_internal::{closure_env#2}, isize> (panic.rs:137)
==41800==    by 0x129FEB: std::rt::lang_start_internal (rt.rs:148)
==41800==    by 0x111929: std::rt::lang_start (rt.rs:165)
==41800==    by 0x110D0D: main (in /home/wink/prgs/rust/myrepos/exper_allocator_api/target/debug/exper_allocator_api)
==41800== 
main inner: m=Add { left: 5, right: 6 } &*m=0x4a8bf90
==41800== Invalid read of size 8
==41800==    at 0x110BAF: exper_allocator_api::main (main.rs:41)
==41800==    by 0x110D3A: core::ops::function::FnOnce::call_once (function.rs:507)
==41800==    by 0x111C9D: std::sys_common::backtrace::__rust_begin_short_backtrace (backtrace.rs:121)
==41800==    by 0x111950: std::rt::lang_start::{{closure}} (rt.rs:166)
==41800==    by 0x129FEB: call_once<(), (dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (function.rs:606)
==41800==    by 0x129FEB: do_call<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panicking.rs:483)
==41800==    by 0x129FEB: try<i32, &(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (panicking.rs:447)
==41800==    by 0x129FEB: catch_unwind<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panic.rs:137)
==41800==    by 0x129FEB: {closure#2} (rt.rs:148)
==41800==    by 0x129FEB: do_call<std::rt::lang_start_internal::{closure_env#2}, isize> (panicking.rs:483)
==41800==    by 0x129FEB: try<isize, std::rt::lang_start_internal::{closure_env#2}> (panicking.rs:447)
==41800==    by 0x129FEB: catch_unwind<std::rt::lang_start_internal::{closure_env#2}, isize> (panic.rs:137)
==41800==    by 0x129FEB: std::rt::lang_start_internal (rt.rs:148)
==41800==    by 0x111929: std::rt::lang_start (rt.rs:165)
==41800==    by 0x110D0D: main (in /home/wink/prgs/rust/myrepos/exper_allocator_api/target/debug/exper_allocator_api)
==41800==  Address 0x4a8bf90 is 0 bytes inside a block of size 16 free'd
==41800==    at 0x484426F: free (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==41800==    by 0x115206: dealloc (alloc.rs:113)
==41800==    by 0x115206: <alloc::alloc::Global as core::alloc::Allocator>::deallocate (alloc.rs:250)
==41800==    by 0x114551: alloc::alloc::box_free (alloc.rs:348)
==41800==    by 0x1138DB: core::ptr::drop_in_place<alloc::boxed::Box<exper_allocator_api::Protocol>> (mod.rs:490)
==41800==    by 0x112C4E: <exper_allocator_api::MyAllocator as core::alloc::Allocator>::allocate (lib.rs:64)
==41800==    by 0x1111FE: alloc::boxed::Box<T,A>::try_new_uninit_in (boxed.rs:493)
==41800==    by 0x1110C2: alloc::boxed::Box<T,A>::new_uninit_in (boxed.rs:457)
==41800==    by 0x1114F2: alloc::boxed::Box<T,A>::new_in (boxed.rs:388)
==41800==    by 0x110A34: exper_allocator_api::main (main.rs:38)
==41800==    by 0x110D3A: core::ops::function::FnOnce::call_once (function.rs:507)
==41800==    by 0x111C9D: std::sys_common::backtrace::__rust_begin_short_backtrace (backtrace.rs:121)
==41800==    by 0x111950: std::rt::lang_start::{{closure}} (rt.rs:166)
==41800==  Block was alloc'd at
==41800==    at 0x4841888: malloc (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==41800==    by 0x1149AB: alloc::alloc::Global::alloc_impl (alloc.rs:95)
==41800==    by 0x114758: allocate (alloc.rs:237)
==41800==    by 0x114758: alloc::alloc::exchange_malloc (alloc.rs:326)
==41800==    by 0x111EA2: exper_allocator_api::MyAllocator::new (boxed.rs:220)
==41800==    by 0x110934: exper_allocator_api::main (main.rs:34)
==41800==    by 0x110D3A: core::ops::function::FnOnce::call_once (function.rs:507)
==41800==    by 0x111C9D: std::sys_common::backtrace::__rust_begin_short_backtrace (backtrace.rs:121)
==41800==    by 0x111950: std::rt::lang_start::{{closure}} (rt.rs:166)
==41800==    by 0x129FEB: call_once<(), (dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (function.rs:606)
==41800==    by 0x129FEB: do_call<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panicking.rs:483)
==41800==    by 0x129FEB: try<i32, &(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (panicking.rs:447)
==41800==    by 0x129FEB: catch_unwind<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panic.rs:137)
==41800==    by 0x129FEB: {closure#2} (rt.rs:148)
==41800==    by 0x129FEB: do_call<std::rt::lang_start_internal::{closure_env#2}, isize> (panicking.rs:483)
==41800==    by 0x129FEB: try<isize, std::rt::lang_start_internal::{closure_env#2}> (panicking.rs:447)
==41800==    by 0x129FEB: catch_unwind<std::rt::lang_start_internal::{closure_env#2}, isize> (panic.rs:137)
==41800==    by 0x129FEB: std::rt::lang_start_internal (rt.rs:148)
==41800==    by 0x111929: std::rt::lang_start (rt.rs:165)
==41800==    by 0x110D0D: main (in /home/wink/prgs/rust/myrepos/exper_allocator_api/target/debug/exper_allocator_api)
==41800== 
==41800== Invalid read of size 8
==41800==    at 0x110BB2: exper_allocator_api::main (main.rs:41)
==41800==    by 0x110D3A: core::ops::function::FnOnce::call_once (function.rs:507)
==41800==    by 0x111C9D: std::sys_common::backtrace::__rust_begin_short_backtrace (backtrace.rs:121)
==41800==    by 0x111950: std::rt::lang_start::{{closure}} (rt.rs:166)
==41800==    by 0x129FEB: call_once<(), (dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (function.rs:606)
==41800==    by 0x129FEB: do_call<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panicking.rs:483)
==41800==    by 0x129FEB: try<i32, &(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (panicking.rs:447)
==41800==    by 0x129FEB: catch_unwind<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panic.rs:137)
==41800==    by 0x129FEB: {closure#2} (rt.rs:148)
==41800==    by 0x129FEB: do_call<std::rt::lang_start_internal::{closure_env#2}, isize> (panicking.rs:483)
==41800==    by 0x129FEB: try<isize, std::rt::lang_start_internal::{closure_env#2}> (panicking.rs:447)
==41800==    by 0x129FEB: catch_unwind<std::rt::lang_start_internal::{closure_env#2}, isize> (panic.rs:137)
==41800==    by 0x129FEB: std::rt::lang_start_internal (rt.rs:148)
==41800==    by 0x111929: std::rt::lang_start (rt.rs:165)
==41800==    by 0x110D0D: main (in /home/wink/prgs/rust/myrepos/exper_allocator_api/target/debug/exper_allocator_api)
==41800==  Address 0x4a8bf98 is 8 bytes inside a block of size 16 free'd
==41800==    at 0x484426F: free (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==41800==    by 0x115206: dealloc (alloc.rs:113)
==41800==    by 0x115206: <alloc::alloc::Global as core::alloc::Allocator>::deallocate (alloc.rs:250)
==41800==    by 0x114551: alloc::alloc::box_free (alloc.rs:348)
==41800==    by 0x1138DB: core::ptr::drop_in_place<alloc::boxed::Box<exper_allocator_api::Protocol>> (mod.rs:490)
==41800==    by 0x112C4E: <exper_allocator_api::MyAllocator as core::alloc::Allocator>::allocate (lib.rs:64)
==41800==    by 0x1111FE: alloc::boxed::Box<T,A>::try_new_uninit_in (boxed.rs:493)
==41800==    by 0x1110C2: alloc::boxed::Box<T,A>::new_uninit_in (boxed.rs:457)
==41800==    by 0x1114F2: alloc::boxed::Box<T,A>::new_in (boxed.rs:388)
==41800==    by 0x110A34: exper_allocator_api::main (main.rs:38)
==41800==    by 0x110D3A: core::ops::function::FnOnce::call_once (function.rs:507)
==41800==    by 0x111C9D: std::sys_common::backtrace::__rust_begin_short_backtrace (backtrace.rs:121)
==41800==    by 0x111950: std::rt::lang_start::{{closure}} (rt.rs:166)
==41800==  Block was alloc'd at
==41800==    at 0x4841888: malloc (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==41800==    by 0x1149AB: alloc::alloc::Global::alloc_impl (alloc.rs:95)
==41800==    by 0x114758: allocate (alloc.rs:237)
==41800==    by 0x114758: alloc::alloc::exchange_malloc (alloc.rs:326)
==41800==    by 0x111EA2: exper_allocator_api::MyAllocator::new (boxed.rs:220)
==41800==    by 0x110934: exper_allocator_api::main (main.rs:34)
==41800==    by 0x110D3A: core::ops::function::FnOnce::call_once (function.rs:507)
==41800==    by 0x111C9D: std::sys_common::backtrace::__rust_begin_short_backtrace (backtrace.rs:121)
==41800==    by 0x111950: std::rt::lang_start::{{closure}} (rt.rs:166)
==41800==    by 0x129FEB: call_once<(), (dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (function.rs:606)
==41800==    by 0x129FEB: do_call<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panicking.rs:483)
==41800==    by 0x129FEB: try<i32, &(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (panicking.rs:447)
==41800==    by 0x129FEB: catch_unwind<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panic.rs:137)
==41800==    by 0x129FEB: {closure#2} (rt.rs:148)
==41800==    by 0x129FEB: do_call<std::rt::lang_start_internal::{closure_env#2}, isize> (panicking.rs:483)
==41800==    by 0x129FEB: try<isize, std::rt::lang_start_internal::{closure_env#2}> (panicking.rs:447)
==41800==    by 0x129FEB: catch_unwind<std::rt::lang_start_internal::{closure_env#2}, isize> (panic.rs:137)
==41800==    by 0x129FEB: std::rt::lang_start_internal (rt.rs:148)
==41800==    by 0x111929: std::rt::lang_start (rt.rs:165)
==41800==    by 0x110D0D: main (in /home/wink/prgs/rust/myrepos/exper_allocator_api/target/debug/exper_allocator_api)
==41800== 
main inner: bp=Add { left: 5, right: 6 }
main inner:-
deallocate:+ ptr=0x4a8bf90 layout align=8 size=16
==41800== Invalid read of size 1
==41800==    at 0x14B477: core::fmt::num::<impl core::fmt::LowerHex for i8>::fmt (num.rs:155)
==41800==    by 0x11428F: core::fmt::num::<impl core::fmt::Debug for u8>::fmt (num.rs:187)
==41800==    by 0x113E25: <&T as core::fmt::Debug>::fmt (mod.rs:2372)
==41800==    by 0x147C79: {closure#0} (builders.rs:394)
==41800==    by 0x147C79: and_then<(), core::fmt::Error, (), core::fmt::builders::{impl#5}::entry::{closure_env#0}> (result.rs:1372)
==41800==    by 0x147C79: core::fmt::builders::DebugInner::entry (builders.rs:380)
==41800==    by 0x147D38: core::fmt::builders::DebugSet::entry (builders.rs:469)
==41800==    by 0x115952: core::fmt::builders::DebugList::entries (builders.rs:633)
==41800==    by 0x113F2C: <[T] as core::fmt::Debug>::fmt (mod.rs:2598)
==41800==    by 0x113E70: <&T as core::fmt::Debug>::fmt (mod.rs:2372)
==41800==    by 0x148161: run (mod.rs:1256)
==41800==    by 0x148161: core::fmt::write (mod.rs:1224)
==41800==    by 0x12B673: write_fmt<std::io::stdio::StdoutLock> (mod.rs:1682)
==41800==    by 0x12B673: <&std::io::stdio::Stdout as std::io::Write>::write_fmt (stdio.rs:716)
==41800==    by 0x12BD52: write_fmt (stdio.rs:690)
==41800==    by 0x12BD52: print_to<std::io::stdio::Stdout> (stdio.rs:1008)
==41800==    by 0x12BD52: std::io::stdio::_print (stdio.rs:1075)
==41800==    by 0x112F5D: <exper_allocator_api::MyAllocator as core::alloc::Allocator>::deallocate (lib.rs:70)
==41800==  Address 0x4a8bf90 is 0 bytes inside a block of size 16 free'd
==41800==    at 0x484426F: free (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==41800==    by 0x115206: dealloc (alloc.rs:113)
==41800==    by 0x115206: <alloc::alloc::Global as core::alloc::Allocator>::deallocate (alloc.rs:250)
==41800==    by 0x114551: alloc::alloc::box_free (alloc.rs:348)
==41800==    by 0x1138DB: core::ptr::drop_in_place<alloc::boxed::Box<exper_allocator_api::Protocol>> (mod.rs:490)
==41800==    by 0x112C4E: <exper_allocator_api::MyAllocator as core::alloc::Allocator>::allocate (lib.rs:64)
==41800==    by 0x1111FE: alloc::boxed::Box<T,A>::try_new_uninit_in (boxed.rs:493)
==41800==    by 0x1110C2: alloc::boxed::Box<T,A>::new_uninit_in (boxed.rs:457)
==41800==    by 0x1114F2: alloc::boxed::Box<T,A>::new_in (boxed.rs:388)
==41800==    by 0x110A34: exper_allocator_api::main (main.rs:38)
==41800==    by 0x110D3A: core::ops::function::FnOnce::call_once (function.rs:507)
==41800==    by 0x111C9D: std::sys_common::backtrace::__rust_begin_short_backtrace (backtrace.rs:121)
==41800==    by 0x111950: std::rt::lang_start::{{closure}} (rt.rs:166)
==41800==  Block was alloc'd at
==41800==    at 0x4841888: malloc (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==41800==    by 0x1149AB: alloc::alloc::Global::alloc_impl (alloc.rs:95)
==41800==    by 0x114758: allocate (alloc.rs:237)
==41800==    by 0x114758: alloc::alloc::exchange_malloc (alloc.rs:326)
==41800==    by 0x111EA2: exper_allocator_api::MyAllocator::new (boxed.rs:220)
==41800==    by 0x110934: exper_allocator_api::main (main.rs:34)
==41800==    by 0x110D3A: core::ops::function::FnOnce::call_once (function.rs:507)
==41800==    by 0x111C9D: std::sys_common::backtrace::__rust_begin_short_backtrace (backtrace.rs:121)
==41800==    by 0x111950: std::rt::lang_start::{{closure}} (rt.rs:166)
==41800==    by 0x129FEB: call_once<(), (dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (function.rs:606)
==41800==    by 0x129FEB: do_call<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panicking.rs:483)
==41800==    by 0x129FEB: try<i32, &(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (panicking.rs:447)
==41800==    by 0x129FEB: catch_unwind<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panic.rs:137)
==41800==    by 0x129FEB: {closure#2} (rt.rs:148)
==41800==    by 0x129FEB: do_call<std::rt::lang_start_internal::{closure_env#2}, isize> (panicking.rs:483)
==41800==    by 0x129FEB: try<isize, std::rt::lang_start_internal::{closure_env#2}> (panicking.rs:447)
==41800==    by 0x129FEB: catch_unwind<std::rt::lang_start_internal::{closure_env#2}, isize> (panic.rs:137)
==41800==    by 0x129FEB: std::rt::lang_start_internal (rt.rs:148)
==41800==    by 0x111929: std::rt::lang_start (rt.rs:165)
==41800==    by 0x110D0D: main (in /home/wink/prgs/rust/myrepos/exper_allocator_api/target/debug/exper_allocator_api)
==41800== 
deallocate: 0x4a8bf90 [05, 00, 00, 00, 00, 00, 00, 00, 06, 00, 00, 00, 00, 00, 00, 00]
deallocate:  1
deallocate:  2
deallocate:- ptr=0x4a8bf90 layout align=8 size=16
==41800== Invalid free() / delete / delete[] / realloc()
==41800==    at 0x484426F: free (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==41800==    by 0x115206: dealloc (alloc.rs:113)
==41800==    by 0x115206: <alloc::alloc::Global as core::alloc::Allocator>::deallocate (alloc.rs:250)
==41800==    by 0x114551: alloc::alloc::box_free (alloc.rs:348)
==41800==    by 0x1138DB: core::ptr::drop_in_place<alloc::boxed::Box<exper_allocator_api::Protocol>> (mod.rs:490)
==41800==    by 0x11395E: core::ptr::drop_in_place<[alloc::boxed::Box<exper_allocator_api::Protocol>]> (mod.rs:490)
==41800==    by 0x113B5B: <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop (mod.rs:3054)
==41800==    by 0x113A46: core::ptr::drop_in_place<alloc::vec::Vec<alloc::boxed::Box<exper_allocator_api::Protocol>>> (mod.rs:490)
==41800==    by 0x11385A: core::ptr::drop_in_place<core::cell::UnsafeCell<alloc::vec::Vec<alloc::boxed::Box<exper_allocator_api::Protocol>>>> (mod.rs:490)
==41800==    by 0x11383E: core::ptr::drop_in_place<core::cell::RefCell<alloc::vec::Vec<alloc::boxed::Box<exper_allocator_api::Protocol>>>> (mod.rs:490)
==41800==    by 0x110DBA: core::ptr::drop_in_place<exper_allocator_api::MyAllocator> (mod.rs:490)
==41800==    by 0x111C68: alloc::alloc::box_free (alloc.rs:350)
==41800==    by 0x110C83: exper_allocator_api::main (main.rs:46)
==41800==  Address 0x4a8bf90 is 0 bytes inside a block of size 16 free'd
==41800==    at 0x484426F: free (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==41800==    by 0x115206: dealloc (alloc.rs:113)
==41800==    by 0x115206: <alloc::alloc::Global as core::alloc::Allocator>::deallocate (alloc.rs:250)
==41800==    by 0x114551: alloc::alloc::box_free (alloc.rs:348)
==41800==    by 0x1138DB: core::ptr::drop_in_place<alloc::boxed::Box<exper_allocator_api::Protocol>> (mod.rs:490)
==41800==    by 0x112C4E: <exper_allocator_api::MyAllocator as core::alloc::Allocator>::allocate (lib.rs:64)
==41800==    by 0x1111FE: alloc::boxed::Box<T,A>::try_new_uninit_in (boxed.rs:493)
==41800==    by 0x1110C2: alloc::boxed::Box<T,A>::new_uninit_in (boxed.rs:457)
==41800==    by 0x1114F2: alloc::boxed::Box<T,A>::new_in (boxed.rs:388)
==41800==    by 0x110A34: exper_allocator_api::main (main.rs:38)
==41800==    by 0x110D3A: core::ops::function::FnOnce::call_once (function.rs:507)
==41800==    by 0x111C9D: std::sys_common::backtrace::__rust_begin_short_backtrace (backtrace.rs:121)
==41800==    by 0x111950: std::rt::lang_start::{{closure}} (rt.rs:166)
==41800==  Block was alloc'd at
==41800==    at 0x4841888: malloc (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==41800==    by 0x1149AB: alloc::alloc::Global::alloc_impl (alloc.rs:95)
==41800==    by 0x114758: allocate (alloc.rs:237)
==41800==    by 0x114758: alloc::alloc::exchange_malloc (alloc.rs:326)
==41800==    by 0x111EA2: exper_allocator_api::MyAllocator::new (boxed.rs:220)
==41800==    by 0x110934: exper_allocator_api::main (main.rs:34)
==41800==    by 0x110D3A: core::ops::function::FnOnce::call_once (function.rs:507)
==41800==    by 0x111C9D: std::sys_common::backtrace::__rust_begin_short_backtrace (backtrace.rs:121)
==41800==    by 0x111950: std::rt::lang_start::{{closure}} (rt.rs:166)
==41800==    by 0x129FEB: call_once<(), (dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (function.rs:606)
==41800==    by 0x129FEB: do_call<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panicking.rs:483)
==41800==    by 0x129FEB: try<i32, &(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (panicking.rs:447)
==41800==    by 0x129FEB: catch_unwind<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panic.rs:137)
==41800==    by 0x129FEB: {closure#2} (rt.rs:148)
==41800==    by 0x129FEB: do_call<std::rt::lang_start_internal::{closure_env#2}, isize> (panicking.rs:483)
==41800==    by 0x129FEB: try<isize, std::rt::lang_start_internal::{closure_env#2}> (panicking.rs:447)
==41800==    by 0x129FEB: catch_unwind<std::rt::lang_start_internal::{closure_env#2}, isize> (panic.rs:137)
==41800==    by 0x129FEB: std::rt::lang_start_internal (rt.rs:148)
==41800==    by 0x111929: std::rt::lang_start (rt.rs:165)
==41800==    by 0x110D0D: main (in /home/wink/prgs/rust/myrepos/exper_allocator_api/target/debug/exper_allocator_api)
==41800== 
main:-
==41800== 
==41800== HEAP SUMMARY:
==41800==     in use at exit: 0 bytes in 0 blocks
==41800==   total heap usage: 13 allocs, 14 frees, 3,229 bytes allocated
==41800== 
==41800== All heap blocks were freed -- no leaks are possible
==41800== 
==41800== For lists of detected and suppressed errors, rerun with: -s
==41800== ERROR SUMMARY: 23 errors from 6 contexts (suppressed: 0 from 0)
```


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
