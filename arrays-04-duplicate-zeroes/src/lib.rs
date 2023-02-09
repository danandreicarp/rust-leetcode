pub fn duplicate_zeros(nums: &mut [i32]) {
    let mut count = 0;

    let mut single_zero = false;
    let size = nums.len();
    for (i, n) in nums.into_iter().enumerate() {
        if i < size - count && *n == 0 {
            count += 1;
            if i == size - count {
                single_zero = true;
            }
        }
    }

    if count == 0 {
        return;
    }

    let mut i = nums.len() - 1;
    while count > 0 && i >= count {
        let idx = if single_zero {
            i - count + 1
        } else {
            i - count
        };

        let temp = nums[idx];
        if temp == 0 {
            if single_zero == false {
                nums[i] = 0;
                if i == 0 {
                    break;
                }
                i = i - 1;
            } else {
                single_zero = false;
            }
            count = count - 1;
        }
        nums[i] = temp;
        if i == 0 {
            break;
        }
        i = i - 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = [1, 0, 2, 3, 0, 4, 5, 0];
        duplicate_zeros(&mut nums);
        assert_eq!(nums, [1, 0, 0, 2, 3, 0, 0, 4]);
    }

    #[test]
    fn test_2() {
        let mut nums = [1, 2, 3];
        duplicate_zeros(&mut nums);
        assert_eq!(nums, [1, 2, 3]);
    }

    #[test]
    fn test_3() {
        let mut nums = [0, 1, 2, 3];
        duplicate_zeros(&mut nums);
        assert_eq!(nums, [0, 0, 1, 2]);
    }

    #[test]
    fn test_4() {
        let mut nums = [0, 0, 1, 0, 2, 3];
        duplicate_zeros(&mut nums);
        assert_eq!(nums, [0, 0, 0, 0, 1, 0]);
    }

    #[test]
    fn test_5() {
        let mut nums = [0];
        duplicate_zeros(&mut nums);
        assert_eq!(nums, [0]);
    }

    #[test]
    fn test_6() {
        let mut nums = [];
        duplicate_zeros(&mut nums);
        assert_eq!(nums, []);
    }
}
