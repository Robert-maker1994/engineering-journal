# Binary Trees. 

## Introduction
A different type of tree, A binary tree is a hierarchical data structure where each node has at most two children, referred to as the left child and the right child.

## Rules for all binary trees. 
- A *binary structure* each node can have at most two children: a left child and a right child.
- There is a single root node (the topmost node). Every child node is connected to one parent node (except the root).
- Each left and right child is itself the root of its own subtree, which is also a binary tree.


## There are different types of binary trees. 

### 1. Full Binary Tree 
- Every node has either 0 or 2 children.
- No node can have only one child. 
```markdown
        1
      /   \
     2     3
    / \
   4   5
```

### 2. Complete Binary Tree:
- All levels are fully filled except possibly the last.
- In the last level, nodes are as far left as possible.
```markdown
        1
      /   \
     2     3
    / \   /
   4   5 6
```

### 3. Perfect Binary Tree:
- All internal nodes have exactly 2 children. As you can how perfect it looks below. 

```markdown
        1
      /   \
     2     3
    / \   / \
   4   5 6   7
```

### 4. Balanced Binary Tree 
- For every node, the height difference between the left and right subtrees is at most 1.
- Ensures efficient operations like searching.
```markdown
        10
      /    \
     5      15
    / \    / 
   3   7  12  

```


### 4. Unbalanced Binary Tree 
- The oppersite of a Balanced Tree if the left and right has a difference of above one. It would be an unbalanced tree
- Ensures efficient operations like searching.
```markdown
        10
      /    \
     5      15
    / \    
   3   7  
  /
 4
```

### 5. Binary Search Tree (BST)
- For every node:
    - All values in the left subtree are less than the node's value.
    - All values in the right subtree are greater than the node's value.
- Ensures efficient searching, insertion, and deletion

The rules for this binary search improves the sorting and 

```
        10
      /    \
     5      15
    / \       \
   3   7      20
// All left nodes in the subtree of 5 are smaller than 5.
// All right nodes in the subtree of 5 are greater than 5.

```
