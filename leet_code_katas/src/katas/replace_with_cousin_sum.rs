use std::rc::Rc;
use std::cell::RefCell;
use super::solution::Solution;
use super::tree_node::TreeNode;
impl Solution {
// DFS, or Depth-First Search, is a fundamental algorithm used to traverse or search through tree or graph data structures. The algorithm starts at the root (or an arbitrary node in the case of a graph) and explores as far as possible along each branch before backtracking. There are two primary ways to implement DFS: recursively and iteratively (using a stack).

// Key Characteristics of DFS
// Traversal Order: DFS explores nodes by going as deep as possible down one path before backtracking and exploring other paths.
// Backtracking: When DFS reaches a node with no unvisited adjacent nodes, it backtracks to the previous node to explore other unvisited nodes.
// Space Complexity: DFS has a space complexity of O(h) for trees, where h is the height of the tree. For graphs, the space complexity is O(V), where V is the number of vertices.
// DFS in Trees
// In the context of binary trees, DFS can be implemented in three common ways:

// Pre-order: Visit the root node, then recursively visit the left subtree, followed by the right subtree.
// In-order: Recursively visit the left subtree, then visit the root node, followed by the right subtree.
// Post-order: Recursively visit the left subtree, then the right subtree, and finally visit the root node.
// Example of DFS in Rust
// Here is an example of how DFS can be implemented in Rust for a binary tree:
// Definition for a binary tree node
    pub fn replace_with_cousin_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut depth_sum = Vec::new();

        // First DFS to calculate the sum of node values at each depth
        fn dfs1(node: Option<Rc<RefCell<TreeNode>>>, d: usize, depth_sum: &mut Vec<i32>) {
            if let Some(node_rc) = node {
                let node_ref = node_rc.borrow();
                if d >= depth_sum.len() {
                    depth_sum.push(node_ref.val);
                } else {
                    depth_sum[d] += node_ref.val;
                }
                dfs1(node_ref.left.clone(), d + 1, depth_sum);
                dfs1(node_ref.right.clone(), d + 1, depth_sum);
            }
        }

        // Second DFS to replace node values
        fn dfs2(node: Option<Rc<RefCell<TreeNode>>>, val: i32, d: usize, depth_sum: &Vec<i32>) {
            if let Some(node_rc) = node {
                let mut node_ref = node_rc.borrow_mut();
                node_ref.val = val;

                let mut c = if d + 1 < depth_sum.len() {
                    depth_sum[d + 1]
                } else {
                    0
                };

                if let Some(left) = node_ref.left.clone() {
                    c -= left.borrow().val;
                }
                if let Some(right) = node_ref.right.clone() {
                    c -= right.borrow().val;
                }

                dfs2(node_ref.left.clone(), c, d + 1, depth_sum);
                dfs2(node_ref.right.clone(), c, d + 1, depth_sum);
            }
        }

        // First DFS to calculate depth sums
        dfs1(root.clone(), 0, &mut depth_sum);
        // Second DFS to replace values, starting with 0 for the root
        dfs2(root.clone(), 0, 0, &depth_sum);
        // Return the modified root
        root
    }
}
        
