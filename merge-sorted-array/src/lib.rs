pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    if m == 0 {
        if n == 0 {
            return;
        } else {
            std::mem::swap(nums1, nums2);
            return;
        }
    } else if n == 0 {
        return;
    }

    let mut i = (m - 1) as usize;
    let mut j = (n - 1) as usize;
    let mut idx = nums1.len() - 1;

    let mut done = false;

    loop {
        if nums1[i] > nums2[j] {
            nums1[idx] = nums1[i];
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        } else {
            nums1[idx] = nums2[j];
            if j == 0 {
                done = true;
                break;
            } else {
                j -= 1;
            }
        }
        if idx == 0 {
            break;
        } else {
            idx -= 1;
        }
    }

    if !done {
        loop {
            nums1[j] = nums2[j];
            if j > 0 {
                j -= 1;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test_2() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test_3() {
        let mut nums1 = vec![0];
        let m = 0;
        let mut nums2 = vec![1];
        let n = 1;
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test_4() {
        let mut nums1 = vec![2, 0];
        let m = 1;
        let mut nums2 = vec![1];
        let n = 1;
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2]);
    }
}
