use std::alloc::{GlobalAlloc, System};

pub struct Tracing {
    pub inner: System,
}

impl Tracing {
    pub const fn new() -> Self {
        Self { inner: System }
    }
}

unsafe impl GlobalAlloc for Tracing {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        let s = "allocating!\n";
        libc::write(libc::STDOUT_FILENO, s.as_ptr() as _, s.len() as _);
        self.inner.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
        self.inner.dealloc(ptr, layout)
    }
}
