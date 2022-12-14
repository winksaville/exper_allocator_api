#![feature(allocator_api)]
use exper_allocator_api::{MyAllocator, Protocol};


#[allow(unused)]
fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    // Based on: https://stackoverflow.com/a/42186553
    unsafe {
        ::std::slice::from_raw_parts(
            (p as *const T) as *const u8,
            ::std::mem::size_of::<T>(),
        )
    }
}

fn main() {
    println!("main:+");
    {
        println!("main inner:+");

        //#[derive(Debug, PartialEq)]
        //pub enum Protocol {
        //    Add { left: u64, right: u64 },
        //    //Push { left: u64, right: u64 },
        //}
        //
        //let msg = Protocol::Add { left: 5, right: 6 };
        //let p_u8 = any_as_u8_slice(&msg);
        //println!("main inner:  &msg={:p} msg={msg:?} {:02x?}", &msg, p_u8);
        //let bm = Box::new(Protocol::Add { left: 1, right: 2 });
        //let p_u8 = any_as_u8_slice(&*bm);
        //println!("main inner:  &*bm={:p} bm={bm:?} {:02x?}", &*bm, p_u8);

        let ma = MyAllocator::new();
        println!("main inner: 1");
        let pt = Protocol::Add { left: 5, right: 6 };
        println!("main inner: 2");
        let m = Box::<Protocol, MyAllocator>::new_in(pt, ma);
        println!("main inner: 3");
        println!("main inner: m={m:?} &*m={:p}", &*m);
        let bp = *m;
        println!("main inner: bp={bp:?}");
        //assert_eq!(bp, Protocol::Add { left: 5, right: 6 });

        println!("main inner:-");
    }
    println!("main:-");
}
