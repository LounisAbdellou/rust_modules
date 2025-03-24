#[derive(Debug, PartialEq)]
struct Vector<T> {
    x: T,
    y: T,
}

impl<T> Vector<T> {
    fn new(x: T, y: T) -> Self {}
    fn clone(&self) -> Self {}
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
#[test]
fn test_a() {
    let v = Vector {
        x: String::from("Hello, World!"),
        y: String::from("Hello, Rust!"),
    };

    let w = v.clone();

    assert_eq!(&v, &w);
}

#[cfg(test)]
#[test]
fn test_b() {
    let v = Vector::new("Hello, World!", "Hello, Rust!");
    let a = v;
    let b = v;
    assert_eq!(a, b);
}
