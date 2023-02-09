pub fn remove_dup(nums: &mut [i32]) -> i32 {
    nums.len() as i32
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut i = 0;
    let mut j = 1;

    if j < nums.len() {
        'out: loop {
            while nums[i] == nums[j] {
                j += 1;
                if j == nums.len() {
                    break 'out;
                }
            }

            i += 1;
            if i != j {
                nums[i] = nums[j];
            }
        }
    }
    (i + 1) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = vec![1, 1, 2];
        let result = remove_duplicates(&mut nums);
        assert_eq!(result, 2);

        let mut num = [0, 1, 2, 3, 4];
        println!("{}", remove_dup(&mut num));
    }

    #[test]
    fn test_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let result = remove_duplicates(&mut nums);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_3() {
        let mut nums = vec![0, 1, 1, 2, 2, 2, 3, 3, 4, 5, 6];
        let result = remove_duplicates(&mut nums);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_4() {
        let mut nums = vec![0, 1, 2, 3, 4];
        let result = remove_duplicates(&mut nums);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_5() {
        let mut nums = vec![1];
        let result = remove_duplicates(&mut nums);
        assert_eq!(result, 1);
    }
}
