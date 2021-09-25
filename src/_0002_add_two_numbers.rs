use crate::test::MyTest;
use std::option::Option::Some;
use std::ops::{Deref, DerefMut};

pub struct Solution;

// https://leetcode.com/problems/add-two-numbers
impl MyTest for Solution {
    fn test(&self) {
        let l1 = Solution::make_nodes(vec![2, 4, 3]);
        let l2 = Solution::make_nodes(vec![5, 6, 4]);
        let rst = Solution::add_two_numbers(l1, l2);
        println!("rst is: {:?}", Solution::nodes_to_string(&rst));
    }
}

impl Solution {
    fn make_nodes(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut rst = Some(Box::new(ListNode::new(0)));
        let mut current = rst.as_mut();
        for num in nums {
            if let Some(n) = current {
                n.next = Some(Box::new(ListNode::new(num)));
                current = n.next.as_mut();
            }
        }
        rst.unwrap().next
    }

    fn nodes_to_string(node: &Option<Box<ListNode>>) -> String {
        let mut rst = String::new();
        rst.push_str("[");
        let mut current = node;
        while let Some(n) = current {
            rst.push_str(n.val.to_string().as_str());
            rst.push_str(",");
            current = &n.next;
        }
        if rst.len() != 1 {
            rst.truncate(rst.len() - 1);
        }
        rst.push_str("]");
        rst
    }

    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut rst = Some(Box::new(ListNode::new(0)));
        let mut last = rst.as_mut();
        let mut added = 0;
        let mut node1 = l1;
        let mut node2 = l2;
        while node1.is_some() || node2.is_some() || added != 0 {
            if let Some(n) = node1 {
                added += n.val;
                node1 = n.next;
            }
            if let Some(n) = node2 {
                added += n.val;
                node2 = n.next;
            }
            if let Some(n) = last {
                n.next = Some(Box::new(ListNode::new(added % 10)));
                last = n.next.as_mut();
            }
            added /= 10;
        }
        rst.unwrap().next
    }
}


// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}