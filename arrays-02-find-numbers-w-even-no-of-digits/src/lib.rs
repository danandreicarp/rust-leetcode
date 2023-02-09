pub fn find_numbers(nums: &[i32]) -> i32 {
    let mut count = 0;
    for i in nums {
        let digits = count_digits(*i);
        if digits % 2 == 0 {
            count += 1;
        }
    }

    count
}

fn count_digits(mut n: i32) -> u8 {
    let mut count = 0;
    while n / 10 > 0 {
        count += 1;
        n /= 10;
    }
    count + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = find_numbers(&[12, 345, 2, 6, 7896]);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let result = find_numbers(&[555, 901, 482, 1771]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_3() {
        let result = find_numbers(&[1, 2, 3, 4]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_4() {
        let result = find_numbers(&[]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_5() {
        let result = find_numbers(&[11, 1212, 131313, 14141414]);
        assert_eq!(result, 4);
    }
}
