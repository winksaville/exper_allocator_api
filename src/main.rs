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

        let m = Box::<Protocol, MyAllocator>::new_in(pt, ma);
        println!("main inner: m={m:?}");
        assert_eq!(*m, Protocol::Add { left: 5, right: 6 });

        println!("main inner:-");
    }
    println!("main:-");
}
