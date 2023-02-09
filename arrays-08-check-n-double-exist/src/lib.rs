pub fn check_if_exist(nums: &[i32]) -> bool {
    if nums.is_empty() {
        return false;
    }

    let mut i = 0;
    while i < nums.len() - 1 {
        let mut j = i + 1;
        let n = nums[i];
        let is_even = n % 2 == 0;
        let half = n / 2;
        let double = n * 2;

        while j < nums.len() {
            if nums[j] == double || (is_even && nums[j] == half) {
                return true;
            }
            j = j + 1;
        }
        i = i + 1;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = check_if_exist(&[10, 2, 5, 3]);
        assert!(result);
    }

    #[test]
    fn test_2() {
        let result = check_if_exist(&[3, 1, 7, 11]);
        assert!(!result);
    }

    #[test]
    fn test_3() {
        let result = check_if_exist(&[3, 1, 7, 14]);
        assert!(result);
    }

    #[test]
    fn test_4() {
        let result = check_if_exist(&[3]);
        assert!(!result);
    }

    #[test]
    fn test_5() {
        let result = check_if_exist(&[]);
        assert!(!result);
    }

    #[test]
    fn test_6() {
        let result = check_if_exist(&[3, 1]);
        assert!(!result);
    }
}
