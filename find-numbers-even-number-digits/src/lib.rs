pub fn find_numbers(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    for num in nums {
        match num {
            _ if num < 10 => continue,
            _ if num < 100 => { count += 1 }
            _ if num < 1000 => continue,
            _ if num < 10000 => { count += 1; }
            _ if num < 100000 => continue,
            _ if num < 1000000 => { count += 1; }
            _ => panic!("unexpected number {}", num)
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = find_numbers(vec![12, 345, 2, 6, 7896]);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let result = find_numbers(vec![555, 901, 482, 1771]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_3() {
        let result = find_numbers(vec![2, 20, 200, 2000, 20_000, 200_000]);
        assert_eq!(result, 3);
    }
}
