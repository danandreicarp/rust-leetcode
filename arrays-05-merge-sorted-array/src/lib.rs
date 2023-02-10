pub fn merge(nums1: &mut [i32], m: i32, nums2: &mut [i32], n: i32) {
    if n == 0 {
        return;
    }

    let mut j:usize = (n - 1).try_into().unwrap();
    let mut k:usize = (m + n - 1).try_into().unwrap();
    let mut two_done = false;
    if m == 0 {
        copy_remaining(nums1, nums2, j, k, two_done);
        return;
    }

    let mut i:usize = (m - 1).try_into().unwrap();

    loop {
        if nums1[i] > nums2[j] {
            nums1[k] = nums1[i];
            if i == 0 {
                k = k - 1;
                break;
            }
            i = i - 1;
        } else {
            nums1[k] = nums2[j];
            if j == 0 {
                two_done = true;
                k = k - 1;
                break;
            }
            j = j - 1;
        }
        k = k - 1;
    }

    copy_remaining(nums1, nums2, j, k, two_done);    
        
}

fn copy_remaining(nums1: &mut [i32], nums2: &mut [i32], mut j: usize, mut k: usize, two_done: bool) {
    while !two_done {
        nums1[k] = nums2[j];
        if j == 0 {
            break;
        }
        j = j - 1;
        k = k - 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums1 = [1,2,3,0,0,0];
        let mut nums2 = [2,5,6];
        merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, [1,2,2,3,5,6]);
    }

    #[test]
    fn test_2() {
        let mut nums1 = [4,5,6,0,0,0];
        let mut nums2 = [1,2,3];
        merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, [1,2,3,4,5,6]);
    }

    #[test]
    fn test_3() {
        let mut nums1 = [1,3,5,0,0,0];
        let mut nums2 = [2,4,6];
        merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, [1,2,3,4,5,6]);
    }

    #[test]
    fn test_4() {
        let mut nums1 = [2,4,6,0,0,0];
        let mut nums2 = [1,3,5];
        merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, [1,2,3,4,5,6]);
    }

    #[test]
    fn test_5() {
        let mut nums1 = [1];
        let mut nums2 = [];
        merge(&mut nums1, 1, &mut nums2, 0);
        assert_eq!(nums1, [1]);
    }

    #[test]
    fn test_6() {
        let mut nums1 = [0];
        let mut nums2 = [1];
        merge(&mut nums1, 0, &mut nums2, 1);
        assert_eq!(nums1, [1]);
    }
}
