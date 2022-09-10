pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
    if arr.len() < 3 {
        return false;
    }
    let mut peaked = false;

    for i in 1..arr.len() {
        if arr[i] > arr[i - 1] {
            if !peaked {
                continue;
            } else {
                return false;
            }
        } else if arr[i] == arr[i - 1] {
            return false;
        } else {
            if !peaked {
                if i > 1 {
                    peaked = true;
                } else {
                    return false;
                }
            }
            continue;
        }
    }

    peaked
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
}
