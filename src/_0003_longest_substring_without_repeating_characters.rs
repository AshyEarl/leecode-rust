use crate::test::MyTest;

pub struct Solution;

// https://leetcode.com/problems/longest-substring-without-repeating-characters
impl MyTest for Solution {
    fn test(&self) {
        let rst = Solution::length_of_longest_substring("abba".to_string());
        println!("rst is: {:?}", rst);
    }
}

use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_idxs: Vec<i32> = Vec::new();
        // Init index.
        last_idxs.resize(256, -1);
        let mut max = 0;
        // Last no repeate char start index.
        let mut last_repeat_start: i32 = -1;
        for (idx, c) in s.char_indices() {
            let i = idx as i32;
            let no_repeate_len = i - last_repeat_start;
            let last_idx = *last_idxs.get(c as usize).unwrap();
            if last_idx != -1 {
                let mut len = i - last_idx;
                // Ignore: abba (i==3)-> len[4] vs noRepeatLen[2]
                if len > no_repeate_len {
                    len = no_repeate_len;
                }
                // abba (i==3)-> last_repeat_start[1] vs last_idx[0]
                if last_repeat_start < last_idx {
                    last_repeat_start = last_idx;
                }
                if len > max {
                    max = len;
                }
            } else {
                if no_repeate_len > max {
                    max = no_repeate_len;
                }
            }
            last_idxs[c as usize] = i;
        }
        max as i32
    }
}
