pub fn is_palindrome(x: i32) -> bool {
    if x < 0 || (x != 0 && x % 10 == 0) {
        false
    } else {
        // return option_1(x);
        // return option_2(x);
        return option_3(x);
    }
}

#[allow(unused)]
fn option_1(x: i32) -> bool {
    let value: String = x.to_string();
    let eulav: String = value.chars().rev().collect();

    value == eulav
}

#[allow(unused)]
fn option_2(x: i32) -> bool {
    let mut length = 1;
    let mut number = x;
    while number / 10 > 0 {
        length += 1;
        number = number / 10;
    }
    println!("number is {} digits long", length);

    let is_palindrome = true;
    for i in 0..length / 2 {
        let a = 10_i32.pow(length - i - 1);
        let b = 10_i32.pow(i + 1);

        let n0 = (x / a) % 10;
        let nn = (x % b) / 10_i32.pow(i);

        if n0 != nn {
            return false;
        }
    }
    true
}

fn option_3(x: i32) -> bool {
    let mut number = x;
    let mut reverse = 0;

    while number > reverse {
        reverse = reverse * 10 + number % 10;
        number = number / 10;
    }

    number == reverse || number == reverse / 10
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        assert!(is_palindrome(121));
    }

    #[test]
    fn test_2() {
        assert!(is_palindrome(123321));
    }

    #[test]
    fn test_3() {
        assert!(!is_palindrome(10));
    }

    #[test]
    fn test_4() {
        assert!(!is_palindrome(100));
    }

    #[test]
    fn test_5() {
        assert!(is_palindrome(1));
    }
}
