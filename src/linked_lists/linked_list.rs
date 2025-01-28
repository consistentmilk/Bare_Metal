use std::alloc::{Allocator, Global};
use std::marker::PhantomData;
use std::ptr::NonNull;

struct Node<T> {
    element: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Node {
            element: elem,
            next: None,
            prev: None,
        }
    }

    fn into_element<A: Allocator>(self: Box<Self, &A>) -> T {
        self.element
    }
}

pub struct LinkedList<T, A: Allocator = Global> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    alloc: A,
    _marker: PhantomData<T>,
}

impl<T, A: Allocator> LinkedList<T, A> {
    //
    //
    // Case 1: Empty list
    //     None(H) None(T)
    //     push_front: [prev, L, next]
    //
    //     End state: None <- [prev, L(H, T), next] -> None
    //
    // Case 2: Non-empty list
    //     None <- [prev, L(H, T), next] -> None
    //     push_front: None <- [prev, LNew, next] -> None
    //
    //     End state: None <- [prev, LNew(H), next] <-> [prev, L(T), next] -> None
    //
    #[inline]
    unsafe fn push_front_node(&mut self, node: NonNull<Node<T>>) {
        // This method takes care not to create mutable references to whole nodes,
        // to maintain validity of aliasing pointers into `element`.
        unsafe {
            (*node.as_ptr()).next = self.head;
            (*node.as_ptr()).prev = None;
            let node = Some(node);

            match self.head {
                None => self.tail = node,
                // Not creating new mutable (unique!) references overlapping `element`.
                Some(head) => (*head.as_ptr()).prev = node,
            }

            self.head = node;
            self.len += 1;
        }
    }

    /// Removes and returns the node at the front of the list.
    #[inline]
    fn pop_front_node(&mut self) -> Option<Box<Node<T>, &A>> {
        // This method takes care not to create mutable references to whole nodes,
        // to maintain validity of aliasing pointers into `element`.
        self.head.map(|node| unsafe {
            let node = Box::from_raw_in(node.as_ptr(), &self.alloc);
            self.head = node.next;

            match self.head {
                None => self.tail = None,
                // Not creating new mutable (unique!) references overlapping `element`.
                Some(head) => (*head.as_ptr()).prev = None,
            }

            self.len -= 1;
            node
        })
    }

    //
    //
    // Case 1: Empty list
    //     None(H) None(T)
    //     push_front: [prev, L, next]
    //
    //     End state: None <- [prev, L(H, T), next] -> None
    //
    // Case 2: Non-empty list
    //      None <- [prev, L(H, T), next] -> None
    //      push_back: [prev, LNew, next]
    //
    //      End state: None <- [prev, L(H), next] <-> [prev, LNew(T), next] -> None
    //
    #[inline]
    unsafe fn push_back_node(&mut self, node: NonNull<Node<T>>) {
        unsafe {
            (*node.as_ptr()).next = None;
            (*node.as_ptr()).prev = self.tail;
            let node = Some(node);

            match self.tail {
                None => self.head = node,

                Some(tail) => (*tail.as_ptr()).next = node,
            }

            self.tail = node;
            self.len += 1;
        }
    }

    #[inline]
    fn pop_back_node(&mut self) -> Option<Box<Node<T>, &A>> {
        self.tail.map(|node| unsafe {
            let node = Box::from_raw_in(node.as_ptr(), &self.alloc);
            self.tail = node.prev;

            match self.tail {
                None => self.head = None,

                Some(tail) => (*tail.as_ptr()).next = None,
            }

            self.len -= 1;
            node
        })
    }
}

impl<T> LinkedList<T> {
    pub const fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
            alloc: Global,
            _marker: PhantomData,
        }
    }
}

impl<T, A: Allocator> LinkedList<T, A> {
    pub fn new_in(allocator: A) -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
            alloc: allocator,
            _marker: PhantomData,
        }
    }

    pub fn push_front(&mut self, elt: T) {
        let node = Box::new_in(Node::new(elt), &self.alloc);
        let node_ptr = NonNull::from(Box::leak(node));

        // SAFETY: node_ptr is a unique pointer to a node we boxed with self.alloc and leaked
        unsafe {
            self.push_front_node(node_ptr);
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node().map(Node::into_element)
    }

    pub fn push_back(&mut self, elt: T) {
        let node = Box::new_in(Node::new(elt), &self.alloc);
        let node_ptr = NonNull::from(Box::leak(node));

        // SAFETY: node_ptr is a unique pointer to a node we boxed with self.alloc and leaked
        unsafe {
            self.push_back_node(node_ptr);
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_back_node().map(Node::into_element)
    }

    pub fn front(&self) -> Option<&T> {
        self.head.map(|node| unsafe { &(*node.as_ptr()).element })
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.head
            .map(|node| unsafe { &mut (*node.as_ptr()).element })
    }

    pub fn back(&self) -> Option<&T> {
        self.tail.map(|node| unsafe { &(*node.as_ptr()).element })
    }

    pub fn back_mut(&self) -> Option<&mut T> {
        self.tail
            .map(|node| unsafe { &mut (*node.as_ptr()).element })
    }
}

impl<T, A: Allocator> Drop for LinkedList<T, A> {
    fn drop(&mut self) {
        struct DropGuard<'a, T, A: Allocator>(&'a mut LinkedList<T, A>);

        impl<'a, T, A: Allocator> Drop for DropGuard<'a, T, A> {
            fn drop(&mut self) {
                // Continue the same loop we do below. This only runs when a destructor has
                // panicked. If another one panics this will abort.
                while self.0.pop_front_node().is_some() {}
            }
        }

        // Wrap self so that if a destructor panics, we can try to keep looping
        let guard = DropGuard(self);
        while guard.0.pop_front_node().is_some() {}
        std::mem::forget(guard);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llist_1_push_pop_head() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(Some(3), list.pop_front());
    }

    #[test]
    fn test_llist_2_push_pop_tail() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(Some(1), list.pop_front());
        assert_eq!(Some(3), list.pop_back());
    }

    #[test]
    fn test_llist_3_front_tail() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(Some(&3), list.front());
        assert_eq!(Some(&1), list.back());

        list.front_mut().map(|x: &mut i32| *x += 10);
        assert_eq!(Some(&13), list.front());

        list.back_mut().map(|x: &mut i32| *x += 10);
        assert_eq!(Some(&11), list.back());
    }
}
