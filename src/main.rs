pub mod alloc;
pub mod sample;

use argh::FromArgs;

#[global_allocator]
pub static ALLOCATOR: alloc::Tracing = alloc::Tracing::new();

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

fn main() {
    argh::from_env::<Args>().subcommand.run();
}
