pub fn deduplicate(list: &mut Vec<i32>) {
    let mut i = 0;

    while i < list.len() {
        let mut j = i + 1;

        while j < list.len() {
            if list[j] == list[i] {
                list.remove(j);
            }
            j += 1;
        }
        i += 1;
    }
}

#[cfg(test)]
#[test]
fn basic_test() {
    let mut v = vec![1, 2, 2, 3, 2, 4, 3];
    deduplicate(&mut v);
    assert_eq!(v, [1, 2, 3, 4]);
}
