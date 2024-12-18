fn min(a: i32, b: i32) -> i32 {
    let value = if a < b { a } else { b };

    value
}

fn main() {
    let n = min(42, 84);

    println!("{}", n);
}
