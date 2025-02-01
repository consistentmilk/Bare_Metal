use std::cell::{RefCell, RefMut};
use std::rc::{Rc, Weak};

pub struct Node<T>
where
    T: Copy,
{
    pub value: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
    pub prev: Option<Weak<RefCell<Node<T>>>>,
}

impl<T> Node<T>
where
    T: Copy,
{
    pub fn new(value: T) -> Self {
        Self {
            value,
            next: None,
            prev: None,
        }
    }
}

impl<T> From<Node<T>> for Option<Rc<RefCell<Node<T>>>>
where
    T: Copy,
{
    fn from(value: Node<T>) -> Self {
        Some(Rc::new(RefCell::new(value)))
    }
}

pub struct LinkedList<T>
where
    T: Copy,
{
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> LinkedList<T>
where
    T: Copy,
{
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn push_back(&mut self, value: T) {
        let mut node: Node<T> = Node::new(value);

        match &mut self.tail.take() {
            None => {
                self.head = node.into();
                self.tail = self.head.clone();
            }

            Some(curr_tail) => {
                node.prev = Some(Rc::downgrade(&curr_tail));
                self.tail = node.into();
                curr_tail.borrow_mut().next = self.tail.clone();
            }
        }
    }

    pub fn push_front(&mut self, value: T) {
        let mut node: Node<T> = Node::new(value);

        match &mut self.head.take() {
            None => {
                self.head = node.into();
                self.tail = self.head.clone();
            }

            Some(curr_head) => {
                node.next = Some(curr_head.clone());
                self.head = node.into();

                if let Some(h) = &self.head {
                    curr_head.borrow_mut().prev = Some(Rc::downgrade(&h));
                }
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match &mut self.tail.take() {
            None => None,
            Some(tail) => {
                let mut tail: RefMut<Node<T>> = tail.borrow_mut();
                let prev: Option<Weak<RefCell<Node<T>>>> = tail.prev.take();

                match prev {
                    None => {
                        self.head.take();
                    }

                    Some(prev) => {
                        let prev: Option<Rc<RefCell<Node<T>>>> = prev.upgrade();

                        if let Some(prev) = prev {
                            prev.borrow_mut().next = None;
                            self.tail = Some(prev);
                        }
                    }
                };

                Some(tail.value)
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match &mut self.head.take() {
            None => None,

            Some(head) => {
                let mut head: RefMut<Node<T>> = head.borrow_mut();
                let next: Option<Rc<RefCell<Node<T>>>> = head.next.take();

                match next {
                    None => {
                        self.tail.take();
                    }

                    Some(next) => {
                        next.borrow_mut().prev = None;
                        self.head = Some(next);
                    }
                }

                Some(head.value)
            }
        }
    }
}

impl<T> Drop for LinkedList<T>
where
    T: Copy,
{
    fn drop(&mut self) {
        while let Some(_) = self.pop_back() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dll_one() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);

        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }
}
