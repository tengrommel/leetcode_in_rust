struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows < 2 {
            return s;
        }
        let seq: Vec<char> = s.chars().collect();
        let unit: usize = (2 * num_rows - 2) as usize;
        let mut string_build = String::with_capacity(s.capacity());
        for row in 0..num_rows {
            let mut i = row as usize;
            let step1 = 2 * (num_rows - 1 - row) as usize;
            let step2 = (unit - step1) as usize;
            let mut trigger = false;
            while i < len {
                string_build.push(seq[i]);
                if step1 == 0 {
                    i += step2
                } else if step2 == 0{
                    i += step1
                } else {
                    i += if trigger { step2 } else {step1};
                    trigger = !trigger;
                }
            }
        }
        string_build
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::convert("PAYPALISHIRING".to_string(), 4),
        "PINALSIGYAHRPI"
    );
    assert_eq!(
        Solution::convert("PAYPALISHIRING".to_string(), 3),
        "PAHNAPLSIIGYIR"
    );
    assert_eq!(Solution::convert("A".to_string(), 1), "A");
    assert_eq!(Solution::convert("AY".to_string(), 2), "AY");
}