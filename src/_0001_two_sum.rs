use crate::test::MyTest;
pub struct Solution;

// https://leetcode.com/problems/two-sum/
impl MyTest for Solution {
    fn test(&self) {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let rst = Solution::two_sum(nums, target);
        println!("rst is: {:?}", rst);
    }
}

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut array = HashMap::new();
        for idx in 0..nums.len() {
            let num = nums[idx];
            let need = target - num;
            // If peer number exist in map, return the index.
            if let Some(found) = array.get(&need) {
                return vec![*found as i32, idx as i32];
            }
            array.insert(num, idx);
        }
        vec![]
    }
}
