#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut number = 0;
        let mut iterator = s.chars().peekable();
        while let Some(ch) = iterator.next() {
            match ch {
                'I' | 'X' | 'C' => {
                    match iterator.peek() {
                        Some(nxt) => match nxt {
                            'V' => {
                                if ch == 'I' {
                                    number += 4;
                                    iterator.next();
                                } else {
                                    number += Solution::process(ch);
                                }
                            }
                            'X' => {
                                if ch == 'I' {
                                    number += 9;
                                    iterator.next();
                                } else {
                                    number += Solution::process(ch);
                                }
                            }
                            'L' => {
                                if ch == 'X' {
                                    number += 40;
                                    iterator.next();
                                } else {
                                    number += Solution::process(ch);
                                }
                            }
                            'C' => {
                                if ch == 'X' {
                                    number += 90;
                                    iterator.next();
                                } else {
                                    number += Solution::process(ch);
                                }
                            }
                            'D' => {
                                if ch == 'C' {
                                    number += 400;
                                    iterator.next();
                                } else {
                                    number += Solution::process(ch);
                                }
                            }
                            'M' => {
                                if ch == 'C' {
                                    number += 900;
                                    iterator.next();
                                } else {
                                    number += Solution::process(ch);
                                }
                            }
                            _ => number += Solution::process(ch),
                        },
                        None => number += Solution::process(ch),
                    };
                }
                _ => number += Solution::process(ch),
            }
        }

        number
    }

    fn process(ch: char) -> i32 {
        match ch {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::roman_to_int("III".to_string()));
    }

    #[test]
    fn test_2() {
        assert_eq!(58, Solution::roman_to_int("LVIII".to_string()));
    }

    #[test]
    fn test_3() {
        assert_eq!(1994, Solution::roman_to_int("MCMXCIV".to_string()));
    }

    #[test]
    fn test_4() {
        assert_eq!(621, Solution::roman_to_int("DCXXI".to_string()));
    }
}
