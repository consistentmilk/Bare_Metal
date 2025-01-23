#![allow(dead_code, unused_imports)]
use std::alloc::{self, Layout};
use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;
use std::mem;
use std::ptr::{self, NonNull};

pub struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
    data: T,
}

pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    _marker: PhantomData<T>,
}

impl<T> Node<T> {
    pub unsafe fn alloc(data: T) -> NonNull<Node<T>> {
        let layout: Layout = Layout::new::<Node<T>>();
        let ptr: *mut Node<T> = alloc::alloc(layout) as *mut Node<T>;

        if ptr.is_null() {
            alloc::handle_alloc_error(layout);
        }

        ptr::write(
            ptr,
            Node {
                next: None,
                prev: None,
                data,
            },
        );

        NonNull::new_unchecked(ptr)
    }

    #[inline]
    pub unsafe fn dealloc(node: NonNull<Node<T>>) {
        alloc::dealloc(node.as_ptr() as *mut u8, Layout::new::<Node<T>>());
    }

    #[inline]
    pub unsafe fn dealloc_drop(mut node: NonNull<Node<T>>) {
        ptr::drop_in_place(&mut node.as_mut().data as *mut T);
        Self::dealloc(node);
    }

    #[inline]
    pub unsafe fn from_ptr<'a>(ptr: Option<NonNull<Node<T>>>) -> Option<&'a Node<T>> {
        ptr.map(|node: NonNull<Node<T>>| &*node.as_ptr())
    }

    #[inline]
    pub unsafe fn from_ptr_mut<'a>(ptr: Option<NonNull<Node<T>>>) -> Option<&'a mut Node<T>> {
        ptr.map(|mut node: NonNull<Node<T>>| mem::transmute::<_, &'a mut Node<T>>(node.as_mut()))
    }
}
