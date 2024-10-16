/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
fn sort<T: PartialOrd + Clone>(array: &mut [T]) {
	//TODO
    let length = array.len();
    if length <= 1 {
        return;
    }

    let mid = length /2 ;

    sort(&mut array[..mid]);
    sort(&mut array[mid..]);

    let (left, right) = array.split_at(mid);
    let mut tmp: Vec<T> = Vec::with_capacity(length);

    let (mut i, mut j) = (0, 0);

    while let (Some(left_val), Some(right_val)) = (left.get(i), right.get(j)) {
        if left_val <= right_val {
            tmp.push(left_val.clone());
            i += 1;
        } else {
            tmp.push(right_val.clone());
            j += 1;
        }
    }

    while let Some(left_val) = left.get(i) {
        tmp.push(left_val.clone());
        i += 1;
    }

    while let Some(right_val) = right.get(j) {
        tmp.push(right_val.clone());
        j += 1;
    }

    for (i, val) in tmp.iter().enumerate() {
        array[i] = val.clone();
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}