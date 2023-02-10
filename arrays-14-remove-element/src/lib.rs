pub fn remove_element(nums: &mut [i32], val: i32) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut k = 0;
    for i in 0..nums.len() {
        if nums[i] != val {
            nums[k] = nums[i];
            k += 1;
        }
    }

    k.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = [3, 2, 2, 3];
        let result = remove_element(&mut nums, 3);
        assert_eq!(nums, [2, 2, 2, 3]);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let mut nums = [];
        let result = remove_element(&mut nums, 3);
        assert_eq!(nums, []);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_3() {
        let mut nums = [3];
        let result = remove_element(&mut nums, 3);
        assert_eq!(nums, [3]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_4() {
        let mut nums = [3, 3];
        let result = remove_element(&mut nums, 3);
        assert_eq!(nums, [3, 3]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_5() {
        let mut nums = [0, 1, 2, 2, 3, 0, 4, 2];
        let result = remove_element(&mut nums, 2);
        assert_eq!(nums, [0, 1, 3, 0, 4, 0, 4, 2]);
        assert_eq!(result, 5);
    }
}
