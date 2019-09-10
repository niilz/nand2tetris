fn main() {
    println!("Hello, world from RUST!");
    use std::env::args;
    let args: Vec<String> = args().collect();
    println!("{:?}", args[1]);
}
