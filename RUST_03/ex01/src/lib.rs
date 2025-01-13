pub fn min<T: PartialOrd>(a: T, b: T) -> T {
    let value = if a < b { a } else { b };

    value
}

#[test]
fn basic_test() {
    assert_eq!(min(12i32, -14i32), -14);
    assert_eq!(min(12f32, 14f32), 12f32);
    assert_eq!(min("abc", "def"), "abc");
    assert_eq!(min(String::from("abc"), String::from("def")), "abc");
}
