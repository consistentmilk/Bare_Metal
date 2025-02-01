use std::alloc::{Allocator, Global, Layout};
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
            allocator,
        }
    }

    // Allocate a new node using the custom allocator
    fn allocate_node(&self, key: K, value: V) -> *mut Node<K, V> {
        let layout = Layout::new::<Node<K, V>>();
        unsafe {
            let ptr = self
                .allocator
                .allocate(layout)
                .unwrap()
                .cast::<Node<K, V>>()
                .as_ptr();
            ptr::write(ptr, Node::new(key, value));
            ptr
        }
    }

    // Deallocate a node using the custom allocator
    unsafe fn deallocate_node(&self, node: *mut Node<K, V>) {
        if !node.is_null() {
            let layout = Layout::new::<Node<K, V>>();
            ptr::drop_in_place(node); // Drop the node's contents
                                      // Convert the raw pointer to NonNull<u8> for deallocation
            if let Some(non_null) = ptr::NonNull::new(node.cast::<u8>()) {
                self.allocator.deallocate(non_null, layout);
            }
        }
    }

    // Insert a key-value pair into the BST (iterative)
    pub fn insert(&mut self, key: K, value: V) {
        unsafe {
            if self.root.is_null() {
                // If the tree is empty, allocate a new node and set it as the root
                self.root = self.allocate_node(key, value);
            } else {
                let mut current = self.root;
                loop {
                    if key < (*current).key {
                        if (*current).left.is_null() {
                            // Allocate a new node and set it as the left child
                            (*current).left = self.allocate_node(key, value);
                            break;
                        } else {
                            current = (*current).left;
                        }
                    } else if key > (*current).key {
                        if (*current).right.is_null() {
                            // Allocate a new node and set it as the right child
                            (*current).right = self.allocate_node(key, value);
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
        }
    }

    // Search for a key in the BST (iterative)
    pub fn search(&self, key: K) -> Option<&V> {
        unsafe {
            let mut current = self.root;
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
        let mut result = Vec::new();
        let mut stack = Vec::new();
        let mut current = self.root;

        unsafe {
            while !current.is_null() || !stack.is_empty() {
                // Traverse to the leftmost node
                while !current.is_null() {
                    stack.push(current);
                    current = (*current).left;
                }
                // Visit the node
                current = stack.pop().unwrap();
                result.push((&(*current).key, &(*current).value));
                // Move to the right subtree
                current = (*current).right;
            }
        }

        result
    }
}

// Drop implementation to clean up memory
impl<K: Ord, V, A: Allocator> Drop for BinarySearchTree<K, V, A> {
    fn drop(&mut self) {
        unsafe {
            let mut stack = Vec::new();
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
        let mut bst = BinarySearchTree::new();
        $(bst.insert($key, $value);)*
        bst
    }};
}
