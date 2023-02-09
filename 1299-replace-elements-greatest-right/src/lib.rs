pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
    if arr.len() == 1 {
        arr[0] = -1;
        return arr;
    }

    let mut i = arr.len() - 1;
    let mut greatest = -1;

    while i > 0 {
        let temp = greatest;
        if arr[i] > greatest {
            greatest = arr[i];
        }
        arr[i] = temp;

        i -= 1;
    }

    arr[i] = greatest;

    return arr;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let arr = vec![17, 18, 5, 4, 6, 1];
        let result = replace_elements(arr);
        assert_eq!(result, vec![18, 6, 6, 6, 1, -1]);
    }

    #[test]
    fn test_2() {
        let arr = vec![400];
        let result = replace_elements(arr);
        assert_eq!(result, vec![-1]);
    }

    #[test]
    fn test_3() {
        let arr = vec![57010, 40840, 69871, 14425, 70605];
        let result = replace_elements(arr);
        assert_eq!(result, vec![70605, 70605, 70605, 70605, -1]);
    }
}
