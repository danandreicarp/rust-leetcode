use std::cmp::max;

pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut max_count = 0;
    let mut count = 0;
    for i in nums {
        if i == 1 { count += 1; }
        else {
            max_count = max(max_count, count);
            count = 0;
        }
    }

    max(max_count, count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let arr = vec![1,1,0,1,1,1];
        let result = find_max_consecutive_ones(arr);
        assert_eq!(result, 3);
    }
}
