// This is from crates.io, demonstrating that we have correctly performed the
// incantations that teach bazel how to fetch libraries from crates.io.
use async_trait::{
    async_trait,
};

#[async_trait]
trait T {}

struct S {}

impl S {
    fn greet(&self) {
        println!("Hello, bazel!");
    }
}

impl T for S {}

fn main() {
    let s = S {};
    s.greet();
}
