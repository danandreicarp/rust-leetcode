pub fn replace_elements(nums: &mut [i32]) -> &[i32] {
    if nums.is_empty() {
        return nums;
    }

    let mut i = nums.len() - 1;
    let mut max = nums[i];
    nums[i] = -1;
    while i > 0 {
        i = i - 1;
        let temp = nums[i];
        nums[i] = max;
        if temp > max {
            max = temp;
        }
    }

    return nums;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = [17, 18, 5, 4, 6, 1];
        let result = replace_elements(&mut nums);
        assert_eq!(result, [18, 6, 6, 6, 1, -1]);
    }

    #[test]
    fn test_2() {
        let mut nums = [1];
        let result = replace_elements(&mut nums);
        assert_eq!(result, [-1]);
    }

    #[test]
    fn test_3() {
        let mut nums = [];
        let result = replace_elements(&mut nums);
        assert_eq!(result, []);
    }

    #[test]
    fn test_4() {
        let mut nums = [1, 2, 3, 4];
        let result = replace_elements(&mut nums);
        assert_eq!(result, [4, 4, 4, -1]);
    }

    #[test]
    fn test_5() {
        let mut nums = [4, 3, 2, 1];
        let result = replace_elements(&mut nums);
        assert_eq!(result, [3, 2, 1, -1]);
    }
}
