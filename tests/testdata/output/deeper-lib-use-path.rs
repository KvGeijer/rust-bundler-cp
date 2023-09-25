pub mod internal {
    pub fn hello_world() {
        println!("Hello, world!");
    }
}
use internal::hello_world;
fn main() {
    hello_world();
    bin_internal::hello();
}
mod bin_internal {
    use crate::internal::hello_world as hello_lib;
    use crate::internal::hello_world;
    pub fn hello() {
        hello_lib();
        hello_world();
    }
}
