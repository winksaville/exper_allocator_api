#![feature(allocator_api)]
use std::{
    alloc::{alloc, dealloc, Allocator, Layout},
    cell::RefCell,
};

#[derive(Debug, PartialEq)]
pub enum Protocol {
    Add { left: u64, right: u64 },
}

#[derive(Debug)]
pub struct MyAllocator {
    data: *mut u8,
    layout: Layout,
    count: usize,
    available: RefCell<Vec<*mut u8>>,
}

impl MyAllocator {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        println!("MyAllocator::new:+");
        const COUNT: usize = 10;
        let protocol_layout = Layout::new::<Protocol>();
        let layout = Layout::new::<[Protocol; COUNT]>();
        let data = unsafe { alloc(layout) };

        if data.is_null() {
            panic!("Failed allocating memory")
        }

        let mut available = vec![];
        for i in 0..COUNT {
            let pp: *mut u8 = ((data as usize) + (i * protocol_layout.size())) as *mut u8;
            available.push(pp);
        }
        let ma = Self {
            data,
            count: COUNT,
            layout,
            available: RefCell::new(available),
        };
        assert_eq!(ma.count, ma.available.borrow().len());
        println!("MyAllocator::new:- {ma:?}");

        ma
    }
}

impl Drop for MyAllocator {
    fn drop(&mut self) {
        println!("MyAllocator::drop:+ {self:?}");
        assert_eq!(self.count, self.available.borrow().len());
        unsafe {
            dealloc(self.data, self.layout);
        }
        println!("MyAllocator::drop:-");
    }
}

unsafe impl Allocator for MyAllocator {
    fn allocate(
        &self,
        layout: std::alloc::Layout,
    ) -> Result<std::ptr::NonNull<[u8]>, std::alloc::AllocError> {
        println!(
            "allocate:+ layout align={} size={} self.available.len={}",
            layout.align(),
            layout.size(),
            self.available.borrow().len(),
        );

        let nnp = if let Some(ptr) = self.available.borrow_mut().pop() {
            println!(
                "allocate:  ptr={ptr:?} layout align={} size={}",
                layout.align(),
                layout.size()
            );
            let ref_array_u8 = unsafe { std::slice::from_raw_parts(ptr, layout.size()) };
            std::ptr::NonNull::<[u8]>::from(ref_array_u8)
        } else {
            panic!("allocate:- No memory available");
        };
        println!(
            "allocate:- ptr={:p} layout align={} size={} self.available.len={}",
            nnp,
            layout.align(),
            layout.size(),
            self.available.borrow().len(),
        );

        Ok(nnp)
    }

    unsafe fn deallocate(&self, ptr: std::ptr::NonNull<u8>, layout: std::alloc::Layout) {
        println!(
            "deallocate:+ ptr={ptr:?} layout align={} size={} self.available.len={}",
            layout.align(),
            layout.size(),
            self.available.borrow().len(),
        );
        let p_mut_u8 = ptr.as_ptr();
        println!("deallocate: p_mut_u8={p_mut_u8:p}");

        self.available.borrow_mut().push(p_mut_u8);
        println!(
            "deallocate:- ptr={ptr:?} layout align={} size={} self.available.len={}",
            layout.align(),
            layout.size(),
            self.available.borrow().len(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_allocation() {
        let ma = MyAllocator::new();
        let msg = Protocol::Add { left: 3, right: 4 };
        let m = Box::<Protocol, MyAllocator>::new_in(msg, ma);
        assert_eq!(*m, Protocol::Add { left: 3, right: 4 });
    }
}
