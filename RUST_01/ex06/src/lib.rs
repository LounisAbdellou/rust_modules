fn convert_to_vector(bytes: &[u8]) -> Vec<u8> {
    let mut i = 0;
    let mut j = 0;
    let mut vector: Vec<u8> = Vec::new();

    while i < bytes.len() && bytes[i] - 48 == 0 {
        i += 1;
    }

    while i + j < bytes.len() {
        if !bytes[i + j].is_ascii_digit() {
            panic!("params must only contain ascii digits");
        }

        vector.push(bytes[i + j]);
        j += 1;
    }

    return vector;
}

pub fn big_add(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    return res;
}

#[cfg(test)]
#[test]
fn basic_test() {
    assert_eq!(big_add(b"2", b"4"), b"6");
    assert_eq!(big_add(b"0010", b"0200"), b"210");
}
