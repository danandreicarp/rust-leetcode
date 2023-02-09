use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set: HashSet<i32> = HashSet::with_capacity(nums.len());

    for i in nums {
        if set.contains(&i) {
            return true;
        } else {
            set.insert(i);
        }
    }

    false
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        let result = contains_duplicate(vec![1, 2, 3, 1]);
        assert!(result);
    }

    #[test]
    fn test_2() {
        let result = contains_duplicate(vec![1, 2, 3, 4]);
        assert!(!result);
    }

    #[test]
    fn test_3() {
        let result = contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]);
        assert!(result);
    }
}
