use std::alloc::{AllocError, Allocator, Global, Layout};
use std::ptr::{self, null_mut};

// Define a node in the BST
#[derive(Debug)]
struct Node<K: Ord, V> {
    key: K,
    value: V,
    left: *mut Node<K, V>,
    right: *mut Node<K, V>,
}

impl<K: Ord, V> Node<K, V> {
    // Create a new node
    fn new(key: K, value: V) -> Self {
        Node {
            key,
            value,
            left: null_mut(),
            right: null_mut(),
        }
    }
}

// Define the BST structure with a custom allocator
#[derive(Debug)]
pub struct BinarySearchTree<K: Ord, V, A: Allocator = Global> {
    root: *mut Node<K, V>,
    len: usize,
    allocator: A,
}

impl<K: Ord, V> BinarySearchTree<K, V> {
    // Create a new BST with the global allocator
    pub fn new() -> Self {
        Self::new_with_allocator(Global)
    }
}

impl<K: Ord, V, A: Allocator> BinarySearchTree<K, V, A> {
    // Create a new BST with a custom allocator
    pub fn new_with_allocator(allocator: A) -> Self {
        BinarySearchTree {
            root: null_mut(),
            len: 0,
            allocator,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    // Allocate a new node using the custom allocator
    fn allocate_node(&self, key: K, value: V) -> Result<*mut Node<K, V>, AllocError> {
        let layout = Layout::new::<Node<K, V>>();

        unsafe {
            // Attempt to allocate memory
            let ptr = self
                .allocator
                .allocate(layout)?
                .cast::<Node<K, V>>()
                .as_ptr();

            // Initialize the node
            ptr::write(ptr, Node::new(key, value));

            Ok(ptr)
        }
    }

    // Deallocate a node using the custom allocator
    unsafe fn deallocate_node(&self, node: *mut Node<K, V>) {
        if !node.is_null() {
            let layout: Layout = Layout::new::<Node<K, V>>();
            unsafe {
                ptr::drop_in_place(node); // Drop the node's contents

                // Convert the raw pointer to NonNull<u8> for deallocation
                if let Some(non_null) = ptr::NonNull::new(node.cast::<u8>()) {
                    self.allocator.deallocate(non_null, layout);
                }
            }
        }
    }

    // Insert a key-value pair into the BST (iterative)
    pub fn insert(&mut self, key: K, value: V) -> Result<(), AllocError> {
        unsafe {
            if self.root.is_null() {
                // If the tree is empty, allocate a new node and set it as the root
                self.root = self.allocate_node(key, value)?;
            } else {
                let mut current = self.root;
                loop {
                    if key < (*current).key {
                        if (*current).left.is_null() {
                            // Allocate a new node and set it as the left child
                            (*current).left = self.allocate_node(key, value)?;
                            break;
                        } else {
                            current = (*current).left;
                        }
                    } else if key > (*current).key {
                        if (*current).right.is_null() {
                            // Allocate a new node and set it as the right child
                            (*current).right = self.allocate_node(key, value)?;
                            break;
                        } else {
                            current = (*current).right;
                        }
                    } else {
                        // Handle duplicate keys (overwrite value)
                        (*current).value = value;
                        break;
                    }
                }
            }

            self.len += 1;

            Ok(())
        }
    }

    // Search for a key in the BST (iterative)
    pub fn search(&self, key: K) -> Option<&V> {
        unsafe {
            let mut current: *mut Node<K, V> = self.root;

            while !current.is_null() {
                if key < (*current).key {
                    current = (*current).left;
                } else if key > (*current).key {
                    current = (*current).right;
                } else {
                    return Some(&(*current).value);
                }
            }

            None
        }
    }

    // In-order traversal (iterative)
    pub fn inorder_traversal(&self) -> Vec<(&K, &V)> {
        let mut result: Vec<(&K, &V)> = Vec::with_capacity(self.len);
        let mut stack: Vec<*mut Node<K, V>> = Vec::with_capacity(self.len);
        let mut current: *mut Node<K, V> = self.root;

        unsafe {
            while !current.is_null() || !stack.is_empty() {
                while !current.is_null() {
                    stack.push(current);

                    current = (*current).left;
                }

                if let Some(node) = stack.pop() {
                    result.push((&(*node).key, &(*node).value));

                    current = (*node).right;
                }
            }
        }

        result
    }
}

// Drop implementation to clean up memory
impl<K: Ord, V, A: Allocator> Drop for BinarySearchTree<K, V, A> {
    fn drop(&mut self) {
        unsafe {
            let mut stack: Vec<*mut Node<K, V>> = Vec::new();

            if !self.root.is_null() {
                stack.push(self.root);
            }

            while let Some(node) = stack.pop() {
                if !(*node).left.is_null() {
                    stack.push((*node).left);
                }

                if !(*node).right.is_null() {
                    stack.push((*node).right);
                }

                self.deallocate_node(node);
            }
        }
    }
}

// Macro to generate a BST from a list of key-value pairs
#[macro_export]
macro_rules! btree {
    ($($key:expr => $value:expr),* $(,)?) => {{
        #[allow(unused)]
        let mut bst = BinarySearchTree::new();
        $(bst.insert($key, $value).ok();)*
        bst
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let bst: BinarySearchTree<i32, &str> = BinarySearchTree::new();
        assert!(bst.root.is_null());
        assert_eq!(bst.search(10), None);
        assert_eq!(bst.inorder_traversal(), vec![]);
    }

    #[test]
    fn test_insert_and_search() {
        let mut bst: BinarySearchTree<i32, &str> = BinarySearchTree::new();
        bst.insert(10, "Ten").ok();
        bst.insert(5, "Five").ok();
        bst.insert(15, "Fifteen").ok();

        assert_eq!(bst.search(10), Some(&"Ten"));
        assert_eq!(bst.search(5), Some(&"Five"));
        assert_eq!(bst.search(15), Some(&"Fifteen"));
        assert_eq!(bst.search(20), None);
    }

    #[test]
    fn test_duplicate_keys() {
        let mut bst: BinarySearchTree<i32, &str> = BinarySearchTree::new();
        bst.insert(10, "Ten").ok();
        bst.insert(10, "New Ten").ok();

        assert_eq!(bst.search(10), Some(&"New Ten"));
    }

    #[test]
    fn test_inorder_traversal() {
        let mut bst: BinarySearchTree<i32, &str> = BinarySearchTree::new();
        bst.insert(10, "Ten").ok();
        bst.insert(5, "Five").ok();
        bst.insert(15, "Fifteen").ok();
        bst.insert(3, "Three").ok();
        bst.insert(7, "Seven").ok();

        let expected: Vec<(&i32, &&str)> = vec![
            (&3, &"Three"),
            (&5, &"Five"),
            (&7, &"Seven"),
            (&10, &"Ten"),
            (&15, &"Fifteen"),
        ];
        assert_eq!(bst.inorder_traversal(), expected);
    }

    #[test]
    fn test_tree_macro() {
        let bst: BinarySearchTree<i32, &str> = btree! {
            10 => "Ten",
            5 => "Five",
            15 => "Fifteen",
            3 => "Three",
            7 => "Seven"
        };

        assert_eq!(bst.search(10), Some(&"Ten"));
        assert_eq!(bst.search(5), Some(&"Five"));
        assert_eq!(bst.search(15), Some(&"Fifteen"));
        assert_eq!(bst.search(3), Some(&"Three"));
        assert_eq!(bst.search(7), Some(&"Seven"));
        assert_eq!(bst.search(20), None);

        let expected: Vec<(&i32, &&str)> = vec![
            (&3, &"Three"),
            (&5, &"Five"),
            (&7, &"Seven"),
            (&10, &"Ten"),
            (&15, &"Fifteen"),
        ];

        assert_eq!(bst.len(), 5);
        assert_eq!(bst.inorder_traversal(), expected);
    }

    #[test]
    fn test_tree_macro_with_duplicates() {
        let bst: BinarySearchTree<i32, &str> = btree! {
            10 => "Ten",
            10 => "New Ten"
        };

        assert_eq!(bst.search(10), Some(&"New Ten"));
    }

    #[test]
    fn test_tree_macro_empty() {
        let bst: BinarySearchTree<i32, &str> = btree! {};
        assert!(bst.root.is_null());
        assert_eq!(bst.inorder_traversal(), vec![]);
    }
}
