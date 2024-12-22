#![crate_type = "lib"]
#![crate_name = "ex03"]

pub fn largest_group<'a, 'b>(haystack: &'a [u32], needle: &'b [u32]) -> &'a [u32] {
    let mut result: &'a [u32] = &[];

    if haystack.is_empty() || needle.is_empty() {
        return result;
    }

    for mut i in 0..haystack.len() {
        let flag = i;

        while i < haystack.len() && needle.contains(&haystack[i]) {
            i += 1;
        }

        if i - flag > 1 && i - flag > result.len() {
            result = &haystack[flag..i];
        }
    }

    return result;
}

#[test]
#[cfg(test)]
fn test_lifetimes() {
    let haystack = [1, 2, 3, 2, 1];
    let result;

    {
        let needle = [2, 3];
        result = largest_group(&haystack, &needle);
    }

    assert_eq!(result, &[2, 3, 2]);
}

#[test]
#[cfg(test)]
fn basic_tests() {
    assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5, 3]), &[3, 5, 5]);
    assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5]), &[5, 5]);
    assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[]), &[]);
    assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[4, 1]), &[]);
}
