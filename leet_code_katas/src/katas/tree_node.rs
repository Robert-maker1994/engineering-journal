#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

/// Creates a new `TreeNode` with the given value.
///
/// # Arguments
///
/// * `val` - An integer value to be assigned to the node.
///
/// # Returns
///
/// A new instance of `TreeNode` with the specified value and `None` for both `left` and `right` children.
///

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