use core::ptr::NonNull;
use std::marker::PhantomData;

use allocator_api2::{
    alloc::{Allocator, Global, Layout},
    boxed::Box,
};

pub struct LinkedList<T, A: Allocator = Global> {
    front: Option<NonNull<Node<T>>>,
    back: Option<NonNull<Node<T>>>,
    len: usize,
    alloc: A,
    _marker: PhantomData<T>,
}

struct Node<T> {
    data: T,
    front: Option<NonNull<Node<T>>>,
    back: Option<NonNull<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self::new_in(Default::default())
    }
}

impl<T, A: Allocator> LinkedList<T, A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            front: None,
            back: None,
            len: 0,
            alloc,
            _marker: PhantomData,
        }
    }

    pub fn push_front(&mut self, val: T) {
        unsafe {
            let new: NonNull<Node<T>> = NonNull::new_unchecked(Box::into_raw(Box::new_in(
                Node {
                    data: val,
                    front: None,
                    back: None,
                },
                &self.alloc,
            )));

            if let Some(old) = self.front {
                (*old.as_ptr()).front = Some(new);
                (*new.as_ptr()).back = Some(old);
            } else {
                self.back = Some(new);
            }

            self.front = Some(new);
            self.len += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        fn into_inner<T, A: Allocator>(boxed: Box<T, A>) -> T {
            let (ptr, alloc) = Box::into_raw_with_allocator(boxed);
            let unboxed: T = unsafe { ptr.read() };

            unsafe {
                let non_null = NonNull::new_unchecked(ptr);
                alloc.deallocate(non_null.cast(), Layout::new::<T>());
            }

            unboxed
        }

        unsafe {
            match self.front {
                None => None,

                Some(node) => {
                    let boxed_node = Box::from_raw_in(node.as_ptr(), &self.alloc);
                    let node = into_inner(boxed_node);
                    let res: T = node.data;

                    self.front = node.back;

                    if let Some(new) = self.front {
                        (*new.as_ptr()).front = None;
                    } else {
                        self.back = None;
                    }

                    self.len -= 1;

                    Some(res)
                }
            }
        }
    }
}

impl<T, A: Allocator + Default> Default for LinkedList<T, A> {
    fn default() -> Self {
        Self::new_in(Default::default())
    }
}

impl<T, A: Allocator> Drop for LinkedList<T, A> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llist_1_push_pop() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(Some(3), list.pop_front());
    }
}
