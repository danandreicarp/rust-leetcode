use std::collections::HashMap;

#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums_set: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            let other_val = target - nums[i];
            if nums_set.contains_key(&other_val) && nums_set[&other_val] != i as i32 {
                return vec![i as i32, nums_set[&other_val]];
            }
            nums_set.insert(nums[i].clone(), i as i32);
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let v = Solution::two_sum(vec![3, 2, 4], 6);

        assert_eq!(v.len(), 2);
        assert!(v.contains(&1));
        assert!(v.contains(&2));
    }

    #[test]
    fn test_2() {
        let v = Solution::two_sum(vec![2, 7, 11, 15], 9);

        assert_eq!(v.len(), 2);
        assert!(v.contains(&0));
        assert!(v.contains(&1));
    }

    #[test]
    fn test_3() {
        let v = Solution::two_sum(vec![0, 4, 3, 0], 0);

        assert_eq!(v.len(), 2);
        assert!(v.contains(&0));
        assert!(v.contains(&3));
    }

    #[test]
    fn test_4() {
        let v = Solution::two_sum(vec![-1, -2, -3, -4, -5], -8);

        assert_eq!(v.len(), 2);
        assert!(v.contains(&2));
        assert!(v.contains(&4));
    }

    #[test]
    fn test_5() {
        let v = Solution::two_sum(vec![-3, 4, 3, 90], 0);

        assert_eq!(v.len(), 2);
        assert!(v.contains(&0));
        assert!(v.contains(&2));
    }
}
