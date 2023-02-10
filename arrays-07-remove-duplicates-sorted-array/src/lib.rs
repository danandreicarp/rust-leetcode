pub fn remove_duplicates(nums: &mut [i32]) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut i = 0;
    let mut j = i + 1;
    while j < nums.len() - 1 {
        if nums[i] == nums[j] {
            j = find_next_index(nums[i], j, nums);
            if j < nums.len() {
                nums[i + 1] = nums[j];
            } else {
                break;
            }
        }
        i = i + 1;
    }

    (i + 1).try_into().unwrap()
}

fn find_next_index(val: i32, mut j: usize, nums: &[i32]) -> usize {
    while j < nums.len() && nums[j] == val {
        j = j + 1;
    }

    return j;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = [1, 1, 2];
        let result = remove_duplicates(&mut nums);
        assert_eq!(result, 2);
        assert_eq!(nums, [1, 2, 2]);
    }

    #[test]
    fn test_2() {
        let mut nums = [0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let result = remove_duplicates(&mut nums);
        assert_eq!(result, 5);
        assert_eq!(nums, [0, 1, 2, 3, 4, 2, 2, 3, 3, 4]);
    }

    #[test]
    fn test_3() {
        let mut nums = [];
        let result = remove_duplicates(&mut nums);
        assert_eq!(result, 0);
        assert!(nums.is_empty());
    }

    #[test]
    fn test_4() {
        let mut nums = [0, 0, 0];
        let result = remove_duplicates(&mut nums);
        assert_eq!(result, 1);
        assert_eq!(nums, [0, 0, 0]);
    }

    #[test]
    fn test_5() {
        let mut nums = [0, 0, 1, 1, 1, 2, 2, 2, 2, 3];
        let result = remove_duplicates(&mut nums);
        assert_eq!(result, 4);
        assert_eq!(nums, [0, 1, 2, 3, 1, 2, 2, 2, 2, 3]);
    }

    #[test]
    fn test_6() {
        let mut nums = [0, 0, 1, 2, 2, 3, 4, 4];
        let result = remove_duplicates(&mut nums);
        assert_eq!(result, 5);
        assert_eq!(nums, [0, 1, 2, 3, 4, 3, 4, 4]);
    }
}
