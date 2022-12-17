#![feature(allocator_api)]
use exper_allocator_api::{ma_init, MyAllocator, Protocol};

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

        ma_init(10);
        let pt = Protocol::Add { left: 5, right: 6 };

        let m = Box::new_in(pt, MyAllocator);
        println!("main inner: m={m:?}");
        assert_eq!(*m, Protocol::Add { left: 5, right: 6 });

        let pt = Protocol::Add { left: 2, right: 4 };

        let m = Box::new_in(pt, MyAllocator);
        println!("main inner: m={m:?}");
        assert_eq!(*m, Protocol::Add { left: 2, right: 4 });

        println!("main inner:-");
    }
    println!("main:-");
}
