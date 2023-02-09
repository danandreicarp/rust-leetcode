use std::{sync::mpsc, thread};

pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
    if arr.len() < 3 {
        return false;
    }

    let mut j = arr.len() - 2;
    let copy = arr.clone();
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut i = 1;
        while arr[i] > arr[i - 1] && i < arr.len() - 1 {
            i += 1;
        }
        tx.send(i).unwrap();
    });

    while copy[j] > copy[j + 1] && j > 0 {
        j -= 1;
    }

    let i = rx.recv().unwrap();

    return j + 1 == i - 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let arr = vec![2, 1];
        let result = valid_mountain_array(arr);
        assert!(!result);
    }

    #[test]
    fn test_2() {
        let arr = vec![0, 1, 2, 3, 1, 2, 3];
        let result = valid_mountain_array(arr);
        assert!(!result);
    }

    #[test]
    fn test_3() {
        let arr = vec![3, 5, 4];
        let result = valid_mountain_array(arr);
        assert!(result);
    }

    #[test]
    fn test_4() {
        let arr = vec![3, 5, 5];
        let result = valid_mountain_array(arr);
        assert!(!result);
    }

    #[test]
    fn test_5() {
        let arr = vec![0, 3, 2, 1];
        let result = valid_mountain_array(arr);
        assert!(result);
    }

    #[test]
    fn test_6() {
        let arr = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = valid_mountain_array(arr);
        assert!(!result);
    }

    #[test]
    fn test_7() {
        let arr = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let result = valid_mountain_array(arr);
        assert!(!result);
    }

    #[test]
    fn test_8() {
        let arr = vec![0, 1, 2, 3, 2, 1];
        let result = valid_mountain_array(arr);
        assert!(result);
    }
}
