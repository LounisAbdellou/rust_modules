fn min(a: i32, b: i32) -> i32 {
    if a < b {
        return a;
    }

    return b;
}

fn main() {
    let n = min(42, 84);

    println!("{}", n);
}
