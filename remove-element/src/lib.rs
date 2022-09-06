pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    if nums.len() == 0 { return 0; }
    
    let mut i = 0;
    let mut j = nums.len() - 1;

    while i <= j {
        if nums[i] == val {
            if nums[j] == val {
                if j > 0 {
                    j -= 1;
                } else {
                    break;
                }
            } else {
                nums[i] = nums[j];
                if j > 0 {
                    j -= 1;
                } else {
                    break;
                }
            }
        } else {
            i += 1;
        }
    }

    i as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = vec![];
        let result = remove_element(&mut nums, 3);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![3];
        let result = remove_element(&mut nums, 3);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_3() {
        let mut nums = vec![3, 2, 2, 3];
        let result = remove_element(&mut nums, 3);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_4() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let result = remove_element(&mut nums, 2);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_5() {
        let mut nums = vec![2];
        let result = remove_element(&mut nums, 3);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_6() {
        let mut nums = vec![3, 3];
        let result = remove_element(&mut nums, 3);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_7() {
        let mut nums = vec![3, 3];
        let result = remove_element(&mut nums, 5);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_8() {
        let mut nums = vec![4, 5];
        let result = remove_element(&mut nums, 4);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_9() {
        let mut nums = vec![1, 2, 3, 4];
        let result = remove_element(&mut nums, 1);
        assert_eq!(result, 3);
    }
}
