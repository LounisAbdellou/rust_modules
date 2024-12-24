pub fn big_add(a: &[u8], b: &[u8]) -> Vec<u8> {}

#[cfg(test)]
#[test]
fn basic_test() {
    assert_eq!(big_add(b"2", b"4"), b"6");
    assert_eq!(big_add(b"0010", b"0200"), b"210");
}
