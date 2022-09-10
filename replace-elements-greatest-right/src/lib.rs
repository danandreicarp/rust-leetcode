pub fn replace_elements(arr: &mut Vec<i32>) -> Vec<i32> {
    if arr.len() == 1 {
        arr[0] = -1;
        return arr.to_owned();
    }

    let mut i = 0;
    while i < arr.len() - 1 {
        let mut max = arr[i + 1];
        for j in (i + 2)..arr.len() - 1 {
            if arr[j] > max {
                max = arr[j];
            }
        }
        arr[i] = max;
        i += 1;
    }
    let last_index = arr.len() - 1;
    arr[last_index] = -1;

    return arr.to_owned();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut arr = vec![17, 18, 5, 4, 6, 1];
        let result = replace_elements(&mut arr);
        assert_eq!(result, vec![18, 6, 6, 6, 1, -1]);
    }

    #[test]
    fn test_2() {
        let mut arr = vec![400];
        let result = replace_elements(&mut arr);
        assert_eq!(result, vec![-1]);
    }
}
