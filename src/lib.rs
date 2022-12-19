#![feature(allocator_api, alloc_layout_extra, nonnull_slice_from_raw_parts)]
use once_cell::sync::Lazy;
use std::{
    alloc::{alloc, Allocator, Layout},
    ptr::NonNull,
    sync::Mutex,
};

#[derive(Debug, PartialEq)]
pub enum Protocol {
    Add { left: u64, right: u64 },
}

#[derive(Debug)]
pub struct MyAllocator;

struct PtrMutU8Wrapper(*mut u8);
unsafe impl Send for PtrMutU8Wrapper {}
unsafe impl Sync for PtrMutU8Wrapper {}

static MA: Lazy<Mutex<Vec<PtrMutU8Wrapper>>> =
    Lazy::new(|| Mutex::new(Vec::<PtrMutU8Wrapper>::new()));

pub fn ma_init(count: usize) {
    //println!("ma_init:+");
    let protocol_layout = Layout::new::<Protocol>();

    // Allocate backing array for MA.
    // SAFETY: see https://doc.rust-lang.org/reference/type-layout.html#array-layout
    //         Arrays are size * N and same alignment as the type. In this case
    // SAFETY: Using unchecked because protocol_layout is safe.
    let data = unsafe {
        alloc(Layout::from_size_align_unchecked(
            protocol_layout.size() * count,
            protocol_layout.align(),
        ))
    };

    if data.is_null() {
        panic!("ma_init:  Failed allocating memory")
    }

    let mut available = MA.lock().unwrap();
    for i in 0..count {
        let p_mut_u8: *mut u8 = ((data as usize) + (i * protocol_layout.size())) as *mut u8;
        available.push(PtrMutU8Wrapper(p_mut_u8));
    }
    //println!("ma_init:- len={}", available.len());
}

unsafe impl Allocator for MyAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, std::alloc::AllocError> {
        // Modeled after: https://github.com/rust-lang/rust/blob/40290505fb0aab2ad673a0caa840fc87a1790338/library/alloc/src/alloc.rs#L173
        match layout.size() {
            0 => Ok(NonNull::slice_from_raw_parts(layout.dangling(), 0)),
            size => {
                let npp = if let Ok(mut ma_locked) = MA.lock() {
                    if let Some(p_mut_u8) = ma_locked.pop() {
                        let ref_array_u8 = unsafe { std::slice::from_raw_parts(p_mut_u8.0, size) };
                        //println!("MyAllocator::allocate: p_mut_u8={ref_array_u8:p}");
                        NonNull::<[u8]>::from(ref_array_u8)
                    } else {
                        panic!("MyAllocator::allocate: Empty MA");
                    }
                } else {
                    panic!("MyAllocator::allocate: Mucked up mutex");
                };

                Ok(npp)
            }
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
        if let Ok(mut ma_locked) = MA.lock() {
            let p_mut_u8 = ptr.as_ptr();
            //println!("MyAllocator::deallocate: p_mut_u8={p_mut_u8:p}");
            ma_locked.push(PtrMutU8Wrapper(p_mut_u8));
        } else {
            panic!("MyAllocator::deallocate: Mucked up mutex");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_allocation() {
        ma_init(10);
        let msg = Protocol::Add { left: 3, right: 4 };
        let m = Box::<Protocol, MyAllocator>::new_in(msg, MyAllocator);
        assert_eq!(*m, Protocol::Add { left: 3, right: 4 });
    }
}
