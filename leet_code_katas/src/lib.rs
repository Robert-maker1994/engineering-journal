use core::str;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node
#[derive(Debug, Clone)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        }))
    }
}
struct Solution;
impl Solution {
    pub fn replace_with_cousin_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
    
        let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, Option<Rc<RefCell<TreeNode>>>)> = VecDeque::new();
        queue.push_back((root.clone().unwrap(), None)); // (node, parent)
    
        while !queue.is_empty() {
            let level_size = queue.len();
            let mut level_nodes = Vec::new();
            let mut parent_map = HashMap::new();
            let mut total_sum = 0;
    
            // Step 1: Collect nodes at the current level and calculate total sum
            for _ in 0..level_size {
                if let Some((node_rc, parent_rc)) = queue.pop_front() {
                    let node = node_rc.borrow();
                    total_sum += node.val;
                    level_nodes.push(node_rc.clone());
                    parent_map.insert(Rc::as_ptr(&node_rc), parent_rc.clone());
    
                    if let Some(left) = node.left.clone() {
                        queue.push_back((left, Some(node_rc.clone())));
                    }
                    if let Some(right) = node.right.clone() {
                        queue.push_back((right, Some(node_rc.clone())));
                    }
                }
            }
    
            // Step 2: Replace each node's value with the cousin sum
            for node_rc in level_nodes {
                let mut node = node_rc.borrow_mut();
                let parent_rc = parent_map.get(&Rc::as_ptr(&node_rc)).unwrap();
                let mut sibling_sum = 0;
    
                // Calculate sibling sum if the node has siblings
                if let Some(parent) = parent_rc {
                    let parent_ref = parent.borrow();
                    if let Some(left) = parent_ref.left.clone() {
                        if Rc::ptr_eq(&left, &node_rc) == false {
                            sibling_sum += left.borrow().val;
                        }
                    }
                    if let Some(right) = parent_ref.right.clone() {
                        if Rc::ptr_eq(&right, &node_rc) == false {
                            sibling_sum += right.borrow().val;
                        }
                    }
                }
    
                // Update the node's value with cousin sum
                node.val = total_sum - sibling_sum - node.val;
            }
        }
    
        root
      
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_with_cousin_sum() {
        

    }

}
