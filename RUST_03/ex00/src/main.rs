fn choose<T>(values: &[T]) -> &T {
    let len = values.len() as i32;
    let index = ftkit::random_number(0..len) as usize;

    return &values[index];
}

fn main() {
    let values = [1, 2, 3, 4, 5];

    println!("{}", choose(&values));
}
