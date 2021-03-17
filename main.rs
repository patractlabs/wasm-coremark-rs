trait Bench {
    /// Load wat bytes and new instance
    fn new(b: &[u8]) -> Self;

    /// Register env
    fn register(&mut self);

    /// Run the coremark
    fn run(&self);
}

fn main() {
    println!("Hello, world");
}
