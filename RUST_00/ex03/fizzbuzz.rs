fn main() {
    for i in 1..101 {
        match (i % 3, i % 5, i % 11) {
            (0, 0, _) => println!("fizzbuzz"),
            (0, _, _) => println!("fizz"),
            (_, 0, _) => println!("buzz"),
            (_, _, 3) => println!("FIZZ"),
            (_, _, 5) => println!("BUZZ"),
            (_, _, _) => println!("{}", i),
        }
    }
}
