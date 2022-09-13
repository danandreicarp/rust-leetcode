pub fn sort_array_by_parity(nums: &mut Vec<i32>) -> Vec<i32> {
    if nums.len() == 1 {
        return nums.to_owned();
    }

    for i in 0..nums.len() {
        if nums[i] % 2 != 0 {
            let mut j = i + 1;
            while j < nums.len() && nums[j] % 2 != 0 {
                j += 1;
            }
            if j < nums.len() {
                let temp = nums[i];
                nums[i] = nums[j];
                nums[j] = temp;
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
        let mut nums = vec![3, 1, 2, 4];
        let result = sort_array_by_parity(&mut nums);
        assert_eq!(result, vec![2, 4, 3, 1]);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![0];
        let result = sort_array_by_parity(&mut nums);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_3() {
        let mut nums = vec![0, 2, 4, 6, 1, 3, 5];
        let result = sort_array_by_parity(&mut nums);
        assert_eq!(result, vec![0, 2, 4, 6, 1, 3, 5]);
    }
}
