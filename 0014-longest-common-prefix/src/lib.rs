#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        // Solution::vertical_scanning(strs)
        Solution::divide_and_conquer(strs)
    }

    fn divide_and_conquer(strs: Vec<String>) -> String {
        let len = strs.len();
        match len {
            0 => "".to_string(),
            1 => strs[0].clone(),
            2 => Solution::lcp(&strs[0], &strs[1]),
            _ => Solution::lcp(
                &Solution::divide_and_conquer(strs[0..len / 2].to_vec()),
                &Solution::divide_and_conquer(strs[len / 2..len].to_vec()),
            ),
        }
    }

    fn lcp(left: &String, right: &String) -> String {
        let mut prefix = 0;
        for (i, c) in left.char_indices() {
            if let Some(oc) = right.chars().nth(i) {
                if c == oc {
                    prefix += 1;
                } else {
                    break;
                }
            }
        }

        if prefix == 0 {
            "".to_string()
        } else {
            left[0..prefix].to_string()
        }
    }

    fn vertical_scanning(strs: Vec<String>) -> String {
        if (strs.is_empty()) {
            return "".to_string();
        }

        let mut i = 0;
        for c in strs[0].chars() {
            for j in 1..strs.len() {
                match strs[j].chars().nth(i) {
                    Some(oc) => {
                        println!("comparing {} with {}", c, oc);
                        if c == oc {
                            continue;
                        }
                    }
                    None => (),
                }

                if (i == 0) {
                    return "".to_string();
                } else {
                    return strs[0][0..i].to_string();
                }
            }
            i += 1;
        }
        return strs[0].clone();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "fl",
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string(),
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "",
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string(),
            ])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "a",
            Solution::longest_common_prefix(vec!["ab".to_string(), "a".to_string(),])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            "",
            Solution::longest_common_prefix(vec!["a".to_string(), "".to_string(),])
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            "c",
            Solution::longest_common_prefix(vec!["cir".to_string(), "car".to_string(),])
        );
    }
}
