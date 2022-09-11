pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut i = 0;
    let mut j = 0;
    while i < nums.len() && j < nums.len() {
        while j < nums.len() && nums[j] == 0 {
            j += 1;
        }
        if j == nums.len() {
            break;
        }
        nums[i] = nums[j];
        i += 1;
        j += 1;
    }

    while i < nums.len() {
        nums[i] = 0;
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![0];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![0]);
    }

    #[test]
    fn test_3() {
        let mut nums = vec![0, 1, 0, 3, 12, -4, 0];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, -4, 0, 0, 0]);
    }
}
