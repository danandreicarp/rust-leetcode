#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut open_parentheses: Vec<char> = Vec::new();
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => {
                    open_parentheses.push(c);
                }
                ')' => match open_parentheses.pop() {
                    None => return false,
                    Some(p) => {
                        if '(' != p {
                            return false;
                        }
                    }
                },
                ']' => match open_parentheses.pop() {
                    None => return false,
                    Some(p) => {
                        if '[' != p {
                            return false;
                        }
                    }
                },
                '}' => match open_parentheses.pop() {
                    None => return false,
                    Some(p) => {
                        if '{' != p {
                            return false;
                        }
                    }
                },
                _ => panic!("unexpected character in input"),
            };
        }

        open_parentheses.is_empty()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_valid("()".to_string()));
    }

    #[test]
    fn test_2() {
        assert!(Solution::is_valid("()[]{}".to_string()));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::is_valid("(]".to_string()));
    }

    #[test]
    fn test_4() {
        assert!(!Solution::is_valid("([)]".to_string()));
    }

    #[test]
    fn test_5() {
        assert!(!Solution::is_valid("[".to_string()));
    }
}
