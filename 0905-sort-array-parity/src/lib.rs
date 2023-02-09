pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
    if nums.len() == 1 {
        return nums.to_owned();
    }

    let mut count_even = 0;
    for num in nums.iter() {
        if num % 2 == 0 {
            count_even += 1;
        }
    }

    let mut j = count_even;
    for i in 0..nums.len() {
        if nums[i] % 2 != 0 {
            while j < nums.len() && nums[j] % 2 != 0 {
                j += 1;
            }
            if j < nums.len() {
                let temp = nums[i];
                nums[i] = nums[j];
                nums[j] = temp;
                j += 1;
            }
            if j == nums.len() {
                break;
            }
        }
    }

    nums.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![3, 1, 2, 4];
        let result = sort_array_by_parity(nums);
        assert_eq!(result, vec![2, 4, 3, 1]);
    }

    #[test]
    fn test_2() {
        let nums = vec![0];
        let result = sort_array_by_parity(nums);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_3() {
        let nums = vec![0, 2, 4, 6, 1, 3, 5];
        let result = sort_array_by_parity(nums);
        assert_eq!(result, vec![0, 2, 4, 6, 1, 3, 5]);
    }

    #[test]
    fn test_4() {
        let nums = vec![0, 1, 2, 3, 4, 5, 6];
        let result = sort_array_by_parity(nums);
        assert_eq!(result, vec![0, 4, 2, 6, 1, 5, 3]);
    }
}
