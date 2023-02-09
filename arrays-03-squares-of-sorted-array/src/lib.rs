pub fn sorted_squares(nums: &mut [i32]) -> &[i32] {
    // for i in nums.into_iter() {
    //     *i = i.pow(2);
    // }
    //nums.sort();

    if nums.is_empty() {
        return nums;
    }

    let mut j = nums.len() - 1;
    loop {
        if nums[0].abs() > nums[j].abs() {
            let temp = nums[j];
            nums[j] = nums[0].pow(2);
            nums[0] = temp;
        } else {
            nums[j] = nums[j].pow(2);
        }
        if j == 0 {
            break;
        }
        j = j - 1;
    }

    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = [-4, -1, 0, 3, 10];
        let result = sorted_squares(&mut nums);
        assert_eq!(result, [0, 1, 9, 16, 100]);
    }

    #[test]
    fn test_2() {
        let mut nums = [-7, -3, 2, 3, 11];
        let result = sorted_squares(&mut nums);
        assert_eq!(result, [4, 9, 9, 49, 121]);
    }

    #[test]
    fn test_3() {
        let mut nums = [];
        let result = sorted_squares(&mut nums);
        assert_eq!(result, []);
    }

    #[test]
    fn test_4() {
        let mut nums = [1, 2, 3, 4];
        let result = sorted_squares(&mut nums);
        assert_eq!(result, [1, 4, 9, 16]);
    }

    #[test]
    fn test_5() {
        let mut nums = [-7, -3, 4, 5, 11];
        let result = sorted_squares(&mut nums);
        assert_eq!(result, [9, 16, 25, 49, 121]);
    }

    #[test]
    fn test_6() {
        let mut nums = [1];
        let result = sorted_squares(&mut nums);
        assert_eq!(result, [1]);
    }
}
