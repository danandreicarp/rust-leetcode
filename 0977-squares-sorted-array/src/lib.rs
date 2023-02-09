pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![-1; nums.len()];

    let mut i = 0;
    let mut j = nums.len() - 1;
    let mut k = nums.len();

    while i < j {
        k -= 1;
        if nums[i].abs() > nums[j].abs() {
            result[k] = nums[i] * nums[i];
            i += 1;
        } else {
            result[k] = nums[j] * nums[j];
            j -= 1;
        }
    }
    result[0] = nums[i] * nums[i];

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = sorted_squares(vec![-4, -1, 0, 3, 10]);
        assert_eq!(result, vec![0, 1, 9, 16, 100]);
    }

    #[test]
    fn test_2() {
        let result = sorted_squares(vec![-7, -3, 2, 3, 11]);
        assert_eq!(result, vec![4, 9, 9, 49, 121]);
    }

    #[test]
    fn test_3() {
        let result = sorted_squares(vec![-7, -3, 4, 5, 11]);
        assert_eq!(result, vec![9, 16, 25, 49, 121]);
    }

    #[test]
    fn test_4() {
        let result = sorted_squares(vec![-10, -1, 0, 3, 4]);
        assert_eq!(result, vec![0, 1, 9, 16, 100]);
    }

    #[test]
    fn test_5() {
        let result = sorted_squares(vec![-4, -4, -3]);
        assert_eq!(result, vec![9, 16, 16]);
    }

    #[test]
    fn test_6() {
        let result = sorted_squares(vec![1]);
        assert_eq!(result, vec![1]);
    }
}
