struct Solution;

impl Solution {
    fn length_of_longest_substrings(s: String) -> i32 {
        let char_list: Vec<char> = s.chars().collect();
        let len = char_list.len();
        let (mut left, mut right, mut max_number) = (0, 0, 0);
        while right < len {
            for index in left..right {
                if char_list[right] == char_list[index] {
                    left = index + 1;
                    break;
                }
            }
            let cur = right - left + 1;
            if cur > max_number {
                max_number = cur;
            }
            right += 1;
        }
        max_number as i32
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::length_of_longest_substrings("abaascb".to_string()),
        4
    );
}
