fn get_boxe_size(boxe: &[u32; 2]) -> u32 {
    return boxe[0] * boxe[1];
}

pub fn sort_boxes(boxes: &mut [[u32; 2]]) {
    if boxes.is_empty() {
        return;
    }

    for i in 1..boxes.len() {
        let tmp = boxes[i];

        for j in (0..=i - 1).rev() {
            if get_boxe_size(&boxes[j]) < get_boxe_size(&tmp) {
                boxes.swap(j + 1, j);
                continue;
            }

            boxes[j + 1] = tmp;
            break;
        }
    }
}

#[cfg(test)]
#[test]
fn sort_test() {
    let mut boxes = [[3, 3], [4, 3], [1, 0], [5, 7], [3, 3]];
    sort_boxes(&mut boxes);
    assert_eq!(boxes, [[5, 7], [4, 3], [3, 3], [3, 3], [1, 0]]);
}
