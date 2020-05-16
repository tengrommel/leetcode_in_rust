struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut input: i64 = x as i64;
        let mut result: i64 = 0;
        let mut digit:i64 = 0;
        let base: i64;
        let upper_bound: i64 = base.pow(31) - 1;
        let lower_bound: i64 = -base.pow(31);
        while input != 0 {
            digit = input % 10;
            result = result * 10 + digit;
            input = input/10;
        }
        if result > upper_bound || result < lower_bound {
            return 0;
        }
        result as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}