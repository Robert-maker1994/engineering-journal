// Definition for a binary tree node
use std::cell::RefCell;
use std::rc::Rc;

use super::DFS;
use crate::katas::data_structures::tree_node::TreeNode;

impl DFS {
    /// This module provides a solution to find the maximum depth of a binary tree.
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
