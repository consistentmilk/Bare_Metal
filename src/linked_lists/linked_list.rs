pub struct ListNode<T> {
    data: T,
    next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    pub fn new(val: T) -> Self {
        Self {
            data: val,
            next: None,
        }
    }
}

pub struct LinkedList<T> {
    head: Option<Box<ListNode<T>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    ///
    /// Init: None(h)
    /// push_front(1): [1, next](h) -> None
    /// push_front(2): [2, next](h) -> [1, next] -> None
    ///
    /// Pushes a new element to the list and,
    ///     -> If list is empty, the new element is made head and its next points to None
    ///     -> If list is non-empty, the new element is made head and its next points to the previous head
    ///
    pub fn push_front(&mut self, val: T) {
        let new_node: Box<ListNode<T>> = Box::new(ListNode {
            data: val,
            next: std::mem::replace(&mut self.head, None),
        });

        self.head = Some(new_node);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match std::mem::replace(&mut self.head, None) {
            None => None,

            Some(node) => {
                self.head = node.next;
                self.len -= 1;

                Some(node.data)
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node: &Box<ListNode<T>>| &node.data)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head
            .as_mut()
            .map(|node: &mut Box<ListNode<T>>| &mut node.data)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter { list: self }
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

pub struct IntoIter<T> {
    list: LinkedList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a ListNode<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            None => None,

            Some(node) => {
                self.next = node.next.as_deref();

                Some(&node.data)
            }
        }
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut ListNode<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match std::mem::replace(&mut self.next, None) {
            None => None,

            Some(node) => {
                self.next = node.next.as_deref_mut();

                Some(&mut node.data)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llist_1_basic_ops() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.pop_front(), Some(3));

        assert_eq!(list.peek(), Some(&2));
        assert_eq!(list.peek_mut(), Some(&mut 2));

        list.peek_mut().map(|val: &mut i32| *val += 10);
        assert_eq!(list.peek(), Some(&12));
    }

    #[test]
    fn test_llist_2_into_iter() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut collector: Vec<i32> = Vec::with_capacity(list.len);

        for node_data in list.into_iter() {
            collector.push(node_data);
        }

        assert_eq!(collector, vec![3, 2, 1]);
    }

    #[test]
    fn test_llist_3_iter() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut it: Iter<'_, i32> = list.iter();
        assert_eq!(Some(&3), it.next());
        assert_eq!(Some(&2), it.next());
        assert_eq!(Some(&1), it.next());
        assert_eq!(None, it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn test_llist_4_iter_mut() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        for node_mut_ptr in list.iter_mut() {
            *node_mut_ptr += 10;
        }

        assert_eq!(vec![13, 12, 11], list.into_iter().collect::<Vec<i32>>());
    }
}
