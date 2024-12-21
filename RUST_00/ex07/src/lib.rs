#![crate_type = "lib"]
#![crate_name = "ex07"]

pub fn strpcmp(query: &[u8], pattern: &[u8]) -> bool {
    let mut i = 0;

    while i < query.len() && i < pattern.len() {
        if query[i] != pattern[i] {
            return false;
        }

        i += 1;
    }

    if i != query.len() || i != pattern.len() {
        return false;
    }

    return true;
}
