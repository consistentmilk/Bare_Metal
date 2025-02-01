//!
//! Implement a doubly-linked list data structure.
//!
//! Associated Structs:
//! 1. Node<T> - Generic
//! 2. LinkedList<T, A> - Generic, Memory allocated using allocator api
//! 3. Iter<'a T, A> - Generic, fixed lifetime
//!
//! Todo:
//! [] Define Node<T>
//! [] Implemented new() and into_element() for Node<T>
//!

use std::alloc::{Allocator, Global};
use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::ptr::NonNull;

struct Node<T> {
    element: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(elt: T) -> Self {
        Node {
            element: elt,
            next: None,
            prev: None,
        }
    }

    pub fn into_element<A: Allocator>(self: Box<Self, &A>) -> T {
        self.element
    }
}

pub struct LinkedList<T, A: Allocator = Global> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    alloc: A,
    marker: PhantomData<T>,
}

impl<T> LinkedList<T> {
    pub const fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
            alloc: Global,
            marker: PhantomData,
        }
    }
}

impl<T> Default for LinkedList<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T, A: Allocator> LinkedList<T, A> {
    pub fn new_in(allocator: A) -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
            alloc: allocator,
            marker: PhantomData,
        }
    }

    #[inline]
    unsafe fn push_front_node(&mut self, node: NonNull<Node<T>>) {
        unsafe {
            (*node.as_ptr()).next = self.head;
            (*node.as_ptr()).prev = None;
            let node: Option<NonNull<Node<T>>> = Some(node);

            match self.head {
                None => self.tail = node,
                Some(old_head) => (*old_head.as_ptr()).prev = node,
            }

            self.head = node;
            self.len += 1;
        }
    }

    #[inline]
    fn pop_front_node(&mut self) -> Option<Box<Node<T>, &A>> {
        self.head.map(|node: NonNull<Node<T>>| unsafe {
            let node: Box<Node<T>, &A> = Box::from_raw_in(node.as_ptr(), &self.alloc);
            self.head = node.next;

            match self.head {
                None => self.tail = None,
                Some(new_head) => (*new_head.as_ptr()).prev = None,
            }

            self.len -= 1;

            node
        })
    }

    #[inline]
    unsafe fn push_back_node(&mut self, node: NonNull<Node<T>>) {
        unsafe {
            (*node.as_ptr()).next = None;
            (*node.as_ptr()).prev = self.tail;
            let node: Option<NonNull<Node<T>>> = Some(node);

            match self.tail {
                None => self.head = node,
                Some(old_tail) => (*old_tail.as_ptr()).next = node,
            }

            self.tail = node;
            self.len += 1;
        }
    }

    #[inline]
    fn pop_back_node(&mut self) -> Option<Box<Node<T>, &A>> {
        self.tail.map(|node: NonNull<Node<T>>| unsafe {
            let node: Box<Node<T>, &A> = Box::from_raw_in(node.as_ptr(), &self.alloc);
            self.tail = node.prev;

            match self.tail {
                None => self.head = None,
                Some(new_tail) => (*new_tail.as_ptr()).next = None,
            }

            self.len -= 1;

            node
        })
    }
}

impl<T, A: Allocator> LinkedList<T, A> {
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push_front(&mut self, elt: T) {
        let new_node: Box<Node<T>, &A> = Box::new_in(Node::new(elt), &self.alloc);
        let node_ptr: NonNull<Node<T>> = NonNull::from(Box::leak(new_node));

        unsafe {
            self.push_front_node(node_ptr);
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node()
            .map(|node: Box<Node<T>, &A>| Node::into_element(node))
    }

    pub fn push_back(&mut self, elt: T) {
        let new_node: Box<Node<T>, &A> = Box::new_in(Node::new(elt), &self.alloc);
        let node_ptr: NonNull<Node<T>> = NonNull::from(Box::leak(new_node));

        unsafe {
            self.push_back_node(node_ptr);
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_back_node()
            .map(|node: Box<Node<T>, &A>| Node::into_element(node))
    }

    pub fn front(&self) -> Option<&T> {
        self.head
            .map(|node: NonNull<Node<T>>| unsafe { &(*node.as_ptr()).element })
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.head
            .map(|node: NonNull<Node<T>>| unsafe { &mut (*node.as_ptr()).element })
    }

    pub fn back(&self) -> Option<&T> {
        self.tail
            .map(|node: NonNull<Node<T>>| unsafe { &(*node.as_ptr()).element })
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.tail
            .map(|node: NonNull<Node<T>>| unsafe { &mut (*node.as_ptr()).element })
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: self.head,
            tail: self.tail,
            len: self.len,
            marker: PhantomData,
        }
    }

    pub fn contains(&self, elt: &T) -> bool
    where
        T: PartialEq,
    {
        self.iter().any(|x| x == elt)
    }
}

impl<T, A: Allocator> Drop for LinkedList<T, A> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}

#[allow(unused)]
pub struct Iter<'a, T: 'a> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node: NonNull<Node<T>>| unsafe {
                let node: &Node<T> = &(*node.as_ptr());
                self.len -= 1;
                self.head = node.next;

                &node.element
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<T> ExactSizeIterator for Iter<'_, T> {}

impl<T> FusedIterator for Iter<'_, T> {}

impl<T> Default for Iter<'_, T> {
    fn default() -> Self {
        Iter {
            head: None,
            tail: None,
            len: 0,
            marker: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_1_push_pop_head() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn testing_2_push_pop_tail() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn testing_3_push_pop_mixed() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_back(3);
        list.push_front(4);
        list.push_back(5); // 4 <-> 2 <-> 1 <-> 3 <-> 5

        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn testing_4_front_borrow() {
        let mut list: LinkedList<u128> = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_back(4);

        assert_eq!(list.front(), Some(&3));

        list.push_front(5);

        assert_eq!(list.front(), Some(&5));

        list.front_mut().map(|node_val: &mut u128| *node_val += 10);

        assert_eq!(list.front(), Some(&15));
        assert_eq!(list.front_mut(), Some(&mut 15));
    }

    #[test]
    fn testing_5_back_borrow() {
        let mut list: LinkedList<u128> = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_back(4);

        assert_eq!(list.back(), Some(&4));

        list.push_back(7);

        assert_eq!(list.back(), Some(&7));

        list.back_mut().map(|node_val: &mut u128| *node_val += 10);

        assert_eq!(list.back(), Some(&17));
        assert_eq!(list.back_mut(), Some(&mut 17));
    }

    #[test]
    fn testing_6_mixed_borrow() {
        let mut list: LinkedList<u128> = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_back(4);

        assert_eq!(list.front(), Some(&3));
        assert_eq!(list.back(), Some(&4));

        list.push_front(5);
        list.push_back(7);

        assert_eq!(list.front(), Some(&5));
        assert_eq!(list.back(), Some(&7));

        list.front_mut().map(|node_val: &mut u128| *node_val += 10);
        list.back_mut().map(|node_val: &mut u128| *node_val += 10);

        assert_eq!(list.front(), Some(&15));
        assert_eq!(list.front_mut(), Some(&mut 15));
        assert_eq!(list.back(), Some(&17));
        assert_eq!(list.back_mut(), Some(&mut 17));
    }

    #[test]
    fn testing_7_iter_next() {
        let mut list: LinkedList<u128> = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_back(4);

        let mut it: Iter<'_, u128> = list.iter();

        assert_eq!(it.next(), Some(&3));
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), Some(&4));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn testing_8_iter_for_syntax() {
        let mut list: LinkedList<u128> = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_back(4);

        let mut collector: Vec<&u128> = Vec::with_capacity(list.len());
        let expected: Vec<&u128> = Vec::from([&3, &2, &1, &4]);

        for node_val in list.iter() {
            collector.push(node_val);
        }

        assert_eq!(expected, collector);
    }

    #[test]
    fn testing_9_iter_collect_method() {
        let mut list: LinkedList<u128> = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_back(4);

        let expected: Vec<&u128> = Vec::from([&3, &2, &1, &4]);

        assert_eq!(expected, list.iter().collect::<Vec<&u128>>());
    }

    #[test]
    fn testing_10_contains_method() {
        let mut list: LinkedList<u128> = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_back(4);
        list.push_front(5);
        list.push_back(7);

        assert!(list.contains(&4));
        assert!(!list.contains(&10));
    }
}
