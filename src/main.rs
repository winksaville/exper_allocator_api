#![feature(allocator_api)]
use exper_allocator_api::{MyAllocator, Protocol};

#[allow(unused)]
fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    // Based on: https://stackoverflow.com/a/42186553
    unsafe {
        ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
    }
}

fn main() {
    println!("main:+");
    {
        println!("main inner:+");

        let ma = MyAllocator::new();
        let pt = Protocol::Add { left: 5, right: 6 };
        // Creating m causes valgrind to generate 2 errors
        let m = Box::<Protocol, MyAllocator>::new_in(pt, ma);

        // Printing causes valgrind two additional errors, 4 total
        println!("main inner: m={m:?}");

        // And asserting causes valgrind two more errors, 6 total
        //assert_eq!(*m, Protocol::Add { left: 5, right: 6 });

        println!("main inner:-");
    }
    println!("main:-");
}
