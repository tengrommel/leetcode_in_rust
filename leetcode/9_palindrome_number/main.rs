struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut digits: Vec<i32> = Vec::new();
        let mut input = x;
        while input != 0 {
            digits.push(input%10);
            input = input/10;
        }
        let len = digits.len();
        // handle one digit
        if len < 2 {
            return ture;
        }
        // handle end with 0
        if digits[0] == 0 {
            return false;
        }
        let mut i = 0;
        while i < len/2 {
            if digits[i] != digits[len - 1- i] {
                return false;
            }
            i += 1;
        }
        true
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_palindrome(9), true);
}