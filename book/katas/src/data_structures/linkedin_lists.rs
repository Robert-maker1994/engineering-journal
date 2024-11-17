//! - **Linked List**: Algorithm and problems related to backtracking
//! Made of nodes
//! - Each node has an value and points to the next. 
//! 
//! Disadvantages
//! - **Memory Overhead**: Each node requires additional memory for storing a reference to the next node.
//! - **Sequential Access**: Accessing elements requires traversing the list from the head to the desired node.
//! - **Cache Performance**: Poor cache performance compared to arrays due to non-contiguous memory allocation.
//! - **Complexity**: Operations such as insertion and deletion require careful handling of pointers.
//! - **Extra Memory for Pointers**: Increased memory usage due to storing pointers in each node.
//! - **No Direct Access**: Elements cannot be accessed directly by index.



struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T: std::fmt::Debug> {
    head: Option<Box<Node<T>>>,
}


impl<T: std::fmt::Debug> LinkedList<T> {
     /// Creates a new `ListNode` with the given value.
    ///
    /// # Arguments
    ///
    /// * `item` - An integer value to be assigned to the node.
    ///
    /// # Returns
    ///
    /// A new instance of `ListNode` with the specified value and `None` for the `next` node.
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    
    /// pushes to the front a new node with the given value to the end of the linked list.
    ///
    /// # Arguments
    ///
    /// * `self` - A mutable reference to the current node.
    /// * `item` - An integer value to be assigned to the new node.
    pub fn push_front(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(), // Take the current head and assign it to the new node
        });
        self.head = Some(new_node); // Update head to the new node
    }

    /// Removes the front node a new node with the given value to the end of the linked list.
    ///
    /// # Arguments
    ///
    /// * `self` - A mutable reference to the current node.
    pub fn pop_front(&mut self) -> Option<T> {

        self.head.take().map(|node| {
            self.head = node.next; // Move the head to the next node
            node.data
        })
    }

    /// Prints the values of all nodes in the linked list.
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the current node.
    pub fn print_list(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{:?} -> ", node.data);
            current = &node.next;
        }
        println!("None");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list() {
        let mut list = LinkedList::new();
        list.push_front(2);
        list.push_front(4);
        list.push_front(3);
        list.print_list();
    }

}

