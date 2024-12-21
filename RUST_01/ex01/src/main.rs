fn min<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
    if *a < *b {
        return a;
    }

    return b;
}

fn main() {
    let a = 42;
    let b = 84;

    println!("min({}, {}) = {}", a, b, min(&a, &b));
}
