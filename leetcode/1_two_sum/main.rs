use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut t: HashMap<i32, usize> = HashMap::new();
        for i in 0..nums.len() {
            t.insert(nums[i], i);
        }
        for i in 0..nums.len() {
            match t.get(&(target - nums[i])) {
                Some(n) => {
                    if i != *n {
                        return vec![i as i32, n.clone() as i32]
                    }
                },
                None => {}
            }
        }
        vec![]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}


