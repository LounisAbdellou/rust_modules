fn merge_vector(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let mut carry = 0;
    let mut vector: Vec<u8> = Vec::new();

    for i in (0..=b.len() - 1).rev() {
        let res = a[i + (a.len() - b.len())] + (b[i] - 48) + carry;

        if res > 57 {
            vector.push(res - 10);
            carry = (res - 48) / 10;
            continue;
        }

        vector.push(res);
        carry = 0;
    }

    if carry > 0 {
        vector.push(carry + 48);
    }

    if a.len() != b.len() {
        for i in 0..b.len() - 1 {
            vector.push(a[i]);
        }
    }

    vector.reverse();

    return vector;
}

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
    let vector_a: Vec<u8> = convert_to_vector(a);
    let vector_b: Vec<u8> = convert_to_vector(b);

    if vector_a.len() > vector_b.len() {
        return merge_vector(&vector_a, &vector_b);
    }

    return merge_vector(&vector_b, &vector_a);
}

#[cfg(test)]
#[test]
fn basic_test() {
    assert_eq!(big_add(b"2", b"4"), b"6");
    assert_eq!(big_add(b"9", b"9"), b"18");
    assert_eq!(big_add(b"99999", b"99999"), b"199998");
    assert_eq!(big_add(b"1009", b"1009"), b"2018");
    assert_eq!(big_add(b"0010", b"0200"), b"210");
    assert_eq!(big_add(b"001234", b"01234"), b"2468");
}
