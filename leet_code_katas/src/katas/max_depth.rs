// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match node {
                Some(node_rc) => {
                    let node_ref = node_rc.borrow();
                    let left_depth = dfs(node_ref.left.clone());
                    let right_depth = dfs(node_ref.right.clone());

                    1 + left_depth.max(right_depth)
                }
                None => 0,
            }
        }
        dfs(root)
    }
}
