fn add(a: &i32, b: i32) -> i32 {
    return *a + b;
}

fn add_assign(a: &mut i32, b: i32) {
    *a += b;
}

fn main() {
    let a = 2;
    let b = 2;
    let mut a_bis = a;

    add_assign(&mut a_bis, b);
    println!("add({}, {}) = {};", a, b, add(&a, b));
    println!("add_assign({}, {}) = {};", a, b, a_bis);
}
