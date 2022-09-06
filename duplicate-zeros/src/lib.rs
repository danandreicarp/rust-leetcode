pub fn duplicate_zeros(arr: &mut Vec<i32>) {
    let mut zeros = 0;
    let mut duplicate_last_zero = false;

    let mut i = 0;
    while i < (arr.len() - zeros - 1) {
        if arr[i] == 0 {
            zeros += 1;
            if i == arr.len() - zeros - 1 {
                // we can fit the duplicate of the final 'zero'
                duplicate_last_zero = true;
            }
        }
        i += 1;
    }

    let mut j = arr.len() - 1;
    while zeros > 0 && j >= zeros {
        if arr[j - zeros] == 0 {
            arr[j] = arr[j - zeros];
            if j == arr.len() - 1 && duplicate_last_zero == false {
                j -= 1;
                continue;
            } else {
                arr[j - 1] = arr[j - zeros];
                zeros -= 1;
                if j > 1 {
                    j -= 2;
                } else if j > 0 {
                    j -= 1;
                }
            }
        } else {
            arr[j] = arr[j - zeros];
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut result = vec![1, 0, 2, 3, 0, 4, 5, 0];
        duplicate_zeros(&mut result);
        assert_eq!(result, vec![1, 0, 0, 2, 3, 0, 0, 4]);
    }

    #[test]
    fn test_2() {
        let mut result = vec![1, 2, 3];
        duplicate_zeros(&mut result);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_3() {
        let mut result = vec![0, 0, 0, 0, 0, 0, 0];
        duplicate_zeros(&mut result);
        assert_eq!(result, vec![0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_4() {
        let mut result = vec![8, 4, 5, 0, 0, 0, 0, 7];
        duplicate_zeros(&mut result);
        assert_eq!(result, vec![8, 4, 5, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_5() {
        let mut result = vec![9, 0, 0, 3, 0, 0, 0];
        duplicate_zeros(&mut result);
        assert_eq!(result, vec![9, 0, 0, 0, 0, 3, 0]);
    }

    #[test]
    fn test_6() {
        let mut result = vec![9, 0, 9, 0, 6, 0, 0, 0, 1];
        duplicate_zeros(&mut result);
        assert_eq!(result, vec![9, 0, 0, 9, 0, 0, 6, 0, 0]);
    }
}
