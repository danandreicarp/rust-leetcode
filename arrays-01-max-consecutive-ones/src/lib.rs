pub fn find_max_consecutive_ones(nums: &[i32]) -> i32 {
    let mut count = 0;
    let mut max = 0;

    for i in nums.into_iter() {
        if *i == 1 {
            count = count + 1;
        } else {
            max = count;
            count = 0;
        }
    }

    max.max(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = find_max_consecutive_ones(&[1, 1, 0, 1, 1, 1]);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let result = find_max_consecutive_ones(&[1, 0, 1, 1, 0, 1]);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_3() {
        let result = find_max_consecutive_ones(&[0, 0, 0]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_4() {
        let result = find_max_consecutive_ones(&[]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_5() {
        let result = find_max_consecutive_ones(&[1, 1, 1, 1]);
        assert_eq!(result, 4);
    }
}
