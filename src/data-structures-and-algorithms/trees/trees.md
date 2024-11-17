# Trees

## Introduction
A tree is a widely used abstract data type that simulates a hierarchical tree structure with a set of connected nodes. Each node contains a value or data and may have a reference to other nodes, forming a parent-child relationship.

## Types of Trees
- [**Binary Tree**](./binary_tree.md): Each node has at most two children.
    - [**Binary Search Tree (BST)**](): A binary tree where the left child contains values less than the parent node, and the right child contains values greater than the parent node.
    - [**Full Binary Tree**](./)
    - [**Complete Binary Tree**]():
- [**Balanced Tree**](): A tree where the height of the left and right subtrees of any node differ by at most one.
- [**Unbalanced Tree**](): A binary search tree with an extra bit of storage per node to ensure the tree remains balanced.

## Terminology
- **Root**: The top node of a tree.
- **Leaf**: A node with no children.
- **Parent**: A node that has one or more children.
- **Child**: A node that has a parent.
- **Subtree**: A tree consisting of a node and its descendants.
- **Depth**: The length of the path from the root to a node.
- **Height**: The length of the path from a node to the deepest leaf.
- **Edge**: The connection between two nodes.

## Key characteristics 
 - Hierarchical structure
 - Non-linear: Unlike linear data structures like arrays or linked lists, trees allow for non-sequential access to data.
 - Nodes can have multiple children: This property enables trees to represent complex relationships between data. 


## Operations
- **Insertion**: Adding a node to the tree.
- **Deletion**: Removing a node from the tree.
- **Traversal**: Visiting all the nodes in a specific order (e.g., in-order, pre-order, post-order).

## Example
```rust
struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}
```

## Conclusion
Trees are fundamental data structures that provide efficient ways to store and manage hierarchical data. Understanding their properties and operations is crucial for solving complex problems in computer science.
