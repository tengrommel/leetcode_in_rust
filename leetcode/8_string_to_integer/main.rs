struct Solution;

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let mut str = str.trim();
        let mut ret: i64 = 0;
        let mut sign: bool = false;
        match str.chars().nth(0) {
            Some(c) => match c {
                '-' => {
                    sign = true;
                    str = &str[1..];
                }
                '+' => {
                    str = &str[1..];
                }
                c if c >= '0' && c <= '9' => {}
                _ => {
                    return 0;
                }
            },
            None => {
                return 0;
            }
        }
        for c in str.chars() {
            if c >= '0' && c <= '9' {
                ret = ret * 10 + c as i64 - '0' as i64;
            } else {
                break;
            }
            match sign {
                true if -ret < i32::MIN as i64 => return i32::MIN,
                false if ret > i32::MAX as i64 => return i32::MAX,
                _ => {}
            }
        }
        if sign {
            ret = -ret
        }
        ret as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::my_atoi("aa".to_string()), 0);
    assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
    assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
    assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
    assert_eq!(Solution::my_atoi("42".to_string()), 42);
    assert_eq!(Solution::my_atoi("004193333".to_string()), 4193333);
}
