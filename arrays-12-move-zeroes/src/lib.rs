pub fn move_zeroes(nums: &mut [i32]) {
    if nums.is_empty() || nums.len() == 1 {
        return;
    }

    let mut k = 0;
    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums[k] = nums[i];
            k = k + 1;
        }
    }

    for i in k..nums.len() {
        nums[i] = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = [0, 1, 0, 3, 12];
        move_zeroes(&mut nums);
        assert_eq!(nums, [1, 3, 12, 0, 0]);
    }

    #[test]
    fn test_2() {
        let mut nums = [0];
        move_zeroes(&mut nums);
        assert_eq!(nums, [0]);
    }

    #[test]
    fn test_3() {
        let mut nums = [];
        move_zeroes(&mut nums);
        assert_eq!(nums, []);
    }

    #[test]
    fn test_4() {
        let mut nums = [1];
        move_zeroes(&mut nums);
        assert_eq!(nums, [1]);
    }

    #[test]
    fn test_5() {
        let mut nums = [1, 0, 1, 0, 0, 1];
        move_zeroes(&mut nums);
        assert_eq!(nums, [1, 1, 1, 0, 0, 0]);
    }
}
