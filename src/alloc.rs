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
        self.inner.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
        self.inner.dealloc(ptr, layout)
    }
}
