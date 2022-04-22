pub mod sample;

use argh::FromArgs;
use std::alloc::{GlobalAlloc, System};

#[derive(FromArgs)]
/// Small string demo
struct Args {
    #[argh(subcommand)]
    subcommand: Subcommand,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum Subcommand {
    Sample(sample::Sample),
}

impl Subcommand {
    fn run(self) {
        match self {
            Subcommand::Sample(x) => x.run(),
        }
    }
}

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

fn main() {
    argh::from_env::<Args>().subcommand.run();
}
