use std::fmt;
use std::ptr;

// Define the Node structure
struct Node<T> {
    value: T,
    next: *mut Node<T>,
}

// Define the LinkedList structure
pub struct LinkedList<T> {
    head: *mut Node<T>,
}

impl<T> LinkedList<T> {
    // Create a new empty linked list
    pub fn new() -> Self {
        LinkedList {
            head: ptr::null_mut(),
        }
    }

    // Add an element to the front of the list
    pub fn push_front(&mut self, value: T) {
        // Allocate memory for the new node
        let new_node = Box::into_raw(Box::new(Node {
            value,
            next: self.head,
        }));
        self.head = new_node;
    }

    // Remove an element from the front of the list
    pub fn pop_front(&mut self) -> Option<T> {
        if self.head.is_null() {
            None
        } else {
            // Take ownership of the head node
            let head = unsafe { Box::from_raw(self.head) };
            self.head = head.next;
            Some(head.value)
        }
    }

    // Check if the list is empty
    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }

    // Get the length of the list
    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = self.head;
        while !current.is_null() {
            count += 1;
            current = unsafe { (*current).next };
        }
        count
    }
}

// Implement Drop to clean up the list
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head;
        while !current.is_null() {
            let next = unsafe { (*current).next };
            unsafe {
                let _ = Box::from_raw(current);
            }; // Drop the node
            current = next;
        }
    }
}

// Implement Display for LinkedList to print the list
impl<T: fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = self.head;
        write!(f, "[")?;
        while !current.is_null() {
            unsafe {
                write!(f, "{}", (*current).value)?;
                current = (*current).next;
                if !current.is_null() {
                    write!(f, ", ")?;
                }
            }
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_new_list_is_empty() {
        let list: LinkedList<i32> = LinkedList::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_list_push_front() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert!(!list.is_empty());
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_list_pop_front() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_list_pop_front_on_empty_list() {
        let mut list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_list_list_length() {
        let mut list = LinkedList::new();
        assert_eq!(list.len(), 0);

        list.push_front(1);
        assert_eq!(list.len(), 1);

        list.push_front(2);
        assert_eq!(list.len(), 2);

        list.pop_front();
        assert_eq!(list.len(), 1);

        list.pop_front();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_list_list_display() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(format!("{}", list), "[3, 2, 1]");
    }

    #[test]
    fn test_list_drop() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // The list should be dropped here, and all nodes should be deallocated
    }

    #[test]
    fn test_list_large_list() {
        let mut list = LinkedList::new();
        for i in 0..1000 {
            list.push_front(i);
        }

        assert_eq!(list.len(), 1000);

        for i in (0..1000).rev() {
            assert_eq!(list.pop_front(), Some(i));
        }

        assert!(list.is_empty());
    }

    #[test]
    fn test_list_string_list() {
        let mut list = LinkedList::new();
        list.push_front("hello".to_string());
        list.push_front("world".to_string());

        assert_eq!(list.pop_front(), Some("world".to_string()));
        assert_eq!(list.pop_front(), Some("hello".to_string()));
        assert_eq!(list.pop_front(), None);
    }
}
