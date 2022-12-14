#![feature(allocator_api)]
use std::{alloc::Allocator, cell::RefCell};

#[derive(Debug, PartialEq)]
pub enum Protocol {
    Add { left: u64, right: u64 },
}

pub struct MyAllocator {
    available: RefCell<Vec<Box<Protocol>>>,
}

impl MyAllocator {
    pub fn new() -> Self {
        let x: RefCell<Vec<Box<Protocol>>> = RefCell::new(vec![]);

        let msg = Box::new(Protocol::Add { left: 0, right: 0 });
        x.borrow_mut().push(msg);

        Self { available: x }
    }
}

unsafe impl Allocator for MyAllocator {

    fn allocate(&self, layout: std::alloc::Layout) -> Result<std::ptr::NonNull<[u8]>, std::alloc::AllocError> {
        println!("allocate:+ layout align={} size={}", layout.align(), layout.size());
        // From: https://stackoverflow.com/a/42186553
        unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
            ::std::slice::from_raw_parts(
                (p as *const T) as *const u8,
                ::std::mem::size_of::<T>(),
            )
        }

        let mut available = self.available.borrow_mut();
        let thing = if let Some(v) = available.pop() {
            println!("allocate: Got one from avaiable");
            v
        } else {
            // Allocate more from the global heap
            println!("allocate: Get a new one");
            let v = Box::new(Protocol::Add { left: 1, right: 2 });
            v
        };
        println!("allocate: thing={thing:?}");

        // Convert and validate size and len are equal
        let ref_array_u8 = unsafe { any_as_u8_slice(&*thing) };
        assert_eq!(layout.size(), ref_array_u8.len());

        // Cast to NonNull<[u8]> and validate address is unchanged
        let ptr = std::ptr::NonNull::from(ref_array_u8);
        unsafe { assert_eq!(ptr.as_ref(), ref_array_u8); }
        println!("allocate: {:p} {:02x?}", ptr, ref_array_u8);

        // Check alignment, we assume layout.align() is power of 2 so
        // align() - 1 is the appropriate mask and masking result is 0.
        let x: usize = ptr.as_ptr() as * const () as usize;
        assert_eq!(0, x & (layout.align() - 1));

        println!("allocate:- ptr={ptr:?} layout align={} size={}", layout.align(), layout.size());
        Ok(ptr)
    }

    unsafe fn deallocate(&self, ptr: std::ptr::NonNull<u8>, layout: std::alloc::Layout) {
        println!("deallocate:+ ptr={ptr:?} layout align={} size={}", layout.align(), layout.size());
        let p_mut_u8 = ptr.as_ptr();
        let p_u8 = std::slice::from_raw_parts(p_mut_u8, layout.size());
        println!("deallocate: {:p} {:02x?}", ptr, p_u8);

        // From: https://stackoverflow.com/questions/28127165/how-to-convert-struct-to-u8/42186553#comment122161841_42186553
        let x: Box<Protocol> = unsafe { std::mem::transmute(ptr)};
        println!("deallocate:  1");

        let mut available = self.available.borrow_mut();
        println!("deallocate:  2");

        available.push(x);
        println!("deallocate:- ptr={ptr:?} layout align={} size={}", layout.align(), layout.size());
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box() {
        let ma = MyAllocator::new();
        let msg = Protocol::Add { left: 3, right: 4 };
        let m = Box::<Protocol, MyAllocator>::new_in(msg, ma);
        assert_eq!(*m, Protocol::Add { left: 3, right: 4 });
    }
}
