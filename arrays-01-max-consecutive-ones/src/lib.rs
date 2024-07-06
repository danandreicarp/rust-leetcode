pub fn find_max_consecutive_ones_original(nums: &[i32]) -> i32 {
    let mut count = 0;
    let mut max = 0;

    for i in nums.into_iter() {
        if *i == 1 {
            count = count + 1;
        } else {
            max = max.max(count);
            count = 0;
        }
    }

    max.max(count)
}

pub fn find_max_consecutive_ones(nums: &[i32]) -> i32 {
    let mut is_in = false;
    let mut start = 0;
    let mut max = 0;

    for i in 0..nums.len() {
        if nums[i] == 0 {
            if is_in {
                is_in = false;
                let length = i - start;
                if length > max {
                    max = length;
                }
            }
        } else {
            if is_in == false {
                is_in = true;
                start = i;
            }
        }
    }

    if is_in {
        max.max(nums.len() - start) as i32
    } else {
        max as i32
    }
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

    #[test]
    fn test_6() {
        let result = find_max_consecutive_ones(&[1,1,1,0,1,1,0]);
        assert_eq!(result, 3);
    }
}
