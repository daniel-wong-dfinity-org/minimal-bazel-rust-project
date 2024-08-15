// This is from crates.io, demonstrating that we have correctly performed the
// incantations that teach bazel how to fetch libraries from crates.io.
use async_trait::{
    async_trait,
};

#[async_trait]
trait Greet {
    fn greet(&self);
}

struct S {}

impl Greet for S {
    fn greet(&self) {
        println!("Hello, bazel and crates.io!");
    }
}

fn main() {
    let s = S {};
    s.greet();
}
