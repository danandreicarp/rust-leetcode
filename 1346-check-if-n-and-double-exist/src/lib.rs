use std::collections::HashSet;

pub fn check_if_exist(arr: Vec<i32>) -> bool {
    let mut doubles = HashSet::new();
    let mut halves = HashSet::new();

    for n in arr {
        if doubles.contains(&n) {
            return true;
        }
        if halves.contains(&n) {
            return true;
        }
        doubles.insert(n * 2);
        if n % 2 == 0 {
            halves.insert(n / 2);
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let arr = vec![10, 2, 5, 3];
        let result = check_if_exist(arr);
        assert!(result);
    }

    #[test]
    fn test_2() {
        let arr = vec![3, 1, 7, 11];
        let result = check_if_exist(arr);
        assert!(!result);
    }

    #[test]
    fn test_3() {
        let arr = vec![4, 1];
        let result = check_if_exist(arr);
        assert!(!result);
    }
}
