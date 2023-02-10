pub fn remove_element(nums: &mut [i32], val: i32) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut i = 0;
    let mut j = nums.len() - 1;

    while i <= j {
        if nums[i] == val {
            let idx = find_next_index(j, nums, val);
            if idx == -1 {
                break;
            } else {
                j = idx.try_into().unwrap();
            }
            if j > i {
                nums[i] = nums[j];
                j = j - 1;
            } else {
                break;
            }
        }
        i = i + 1;
    }

    i.try_into().unwrap()
}

fn find_next_index(mut j: usize, nums: &[i32], val: i32) -> i32 {
    while nums[j] == val {
        if j == 0 {
            return -1;
        }
        j = j - 1;
    }
    return j.try_into().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = [3,2,2,3];
        let result = remove_element(&mut nums, 3);
        assert_eq!(2, result);
        assert_eq!(nums, [2,2,2,3]);
    }

    #[test]
    fn test_2() {
        let mut nums = [0,1,2,2,3,0,4,2];
        let result = remove_element(&mut nums, 2);
        assert_eq!(5, result);
        assert_eq!(nums, [0,1,4,0,3,0,4,2]);
    }

    #[test]
    fn test_3() {
        let mut nums = [];
        let result = remove_element(&mut nums, 3);
        assert_eq!(0, result);
        assert!(nums.is_empty());
    }

    #[test]
    fn test_4() {
        let mut nums = [1];
        let result = remove_element(&mut nums, 1);
        assert_eq!(0, result);
        assert_eq!(nums, [1]);
    }

    #[test]
    fn test_5() {
        let mut nums = [3,2,3,2,3];
        let result = remove_element(&mut nums, 3);
        assert_eq!(2, result);
        assert_eq!(nums, [2,2,3,2,3]);
    }
}
