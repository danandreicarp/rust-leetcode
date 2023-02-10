pub fn remove_duplicates(nums: &mut [i32]) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut k = 1;
    for i in 1..nums.len() {
        if nums[i] != nums[i - 1] {
            nums[k] = nums[i];
            k = k + 1;
        }
    }

    k.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = [1, 1, 2];
        let result = remove_duplicates(&mut nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let mut nums = [];
        let result = remove_duplicates(&mut nums);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_3() {
        let mut nums = [1];
        let result = remove_duplicates(&mut nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_4() {
        let mut nums = [1, 1, 1, 1];
        let result = remove_duplicates(&mut nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_5() {
        let mut nums = [0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let result = remove_duplicates(&mut nums);
        assert_eq!(result, 5);
    }
}
