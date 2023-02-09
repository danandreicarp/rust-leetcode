pub fn valid_mountain_array(nums: &[i32]) -> bool {
    if nums.len() < 3 {
        return false;
    }

    let mut i = 0;
    let mut peaked = false;
    while i < nums.len() - 1 {
        if nums[i] == nums[i + 1] {
            return false;
        } else if !peaked && nums[i] > nums[i + 1] {
            peaked = true;
        } else if peaked && nums[i] < nums[i + 1] {
            return false;
        }
        i = i + 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = valid_mountain_array(&[2, 1]);
        assert!(!result);
    }

    #[test]
    fn test_2() {
        let result = valid_mountain_array(&[3, 5, 5]);
        assert!(!result);
    }

    #[test]
    fn test_3() {
        let result = valid_mountain_array(&[0, 3, 2, 1]);
        assert!(result);
    }

    #[test]
    fn test_4() {
        let result = valid_mountain_array(&[0, 2, 3, 3, 5, 2, 1, 0]);
        assert!(!result);
    }

    #[test]
    fn test_5() {
        let result = valid_mountain_array(&[0, 2, 3, 4, 5, 2, 1, 0]);
        assert!(result);
    }

    #[test]
    fn test_6() {
        let result = valid_mountain_array(&[]);
        assert!(!result);
    }
}
