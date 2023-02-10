pub fn sort_array_by_parity(nums: &mut [i32]) -> &[i32] {
    if nums.is_empty() || nums.len() == 1 {
        return nums;
    }

    let mut i = 0;
    let mut j = nums.len() - 1;
    while i < j {
        while nums[i] % 2 == 0 {
            i += 1;
        }

        while nums[j] % 2 == 1 {
            j -= 1;
        }

        if i < j {
            let temp = nums[i];
            nums[i] = nums[j];
            nums[j] = temp;
            i += 1;
            j -= 1;
        }
    }

    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = [3, 1, 2, 4];
        let result = sort_array_by_parity(&mut nums);
        assert_eq!(result, [4, 2, 1, 3]);
    }

    #[test]
    fn test_2() {
        let mut nums = [];
        let result = sort_array_by_parity(&mut nums);
        assert_eq!(result, []);
    }

    #[test]
    fn test_3() {
        let mut nums = [1];
        let result = sort_array_by_parity(&mut nums);
        assert_eq!(result, [1]);
    }

    #[test]
    fn test_4() {
        let mut nums = [2];
        let result = sort_array_by_parity(&mut nums);
        assert_eq!(result, [2]);
    }

    #[test]
    fn test_5() {
        let mut nums = [2, 4, 6, 5, 3, 1];
        let result = sort_array_by_parity(&mut nums);
        assert_eq!(result, [2, 4, 6, 5, 3, 1]);
    }

    #[test]
    fn test_6() {
        let mut nums = [2, 4, 1, 6, 5, 3];
        let result = sort_array_by_parity(&mut nums);
        assert_eq!(result, [2, 4, 6, 1, 5, 3]);
    }
}
