fn main() {
    println!("Hey! I'm the other target!");

    #[cfg(not(debug_assertions))]
    println!("I'm in release mode!");
}
