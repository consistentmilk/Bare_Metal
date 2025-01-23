#[derive(Debug)]
pub struct CircularQueue<T>
where
    T: Clone + PartialEq + Eq, // Bound required by Option enum
{
    buf: Vec<Option<T>>, // Underlying Vec buffer for the circular queue implementation
    ptr: usize,          // Index of the first enqueued element
    len: usize,          // Currently enqueued total elements
    cap: usize,          // Capacity of the queue
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum QueueError {
    QueueEmpty,
    QueueFull,
}

impl<T> CircularQueue<T>
where
    T: Clone + PartialEq + Eq,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            buf: vec![None; capacity],
            ptr: 0,
            len: 0,
            cap: capacity,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == self.cap
    }

    pub fn enqueue(&mut self, val: T) -> Result<(), QueueError> {
        if self.is_full() {
            return Err(QueueError::QueueFull);
        }

        self.buf[(self.ptr + self.len) % self.cap] = Some(val);
        self.len += 1;

        Ok(())
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let dequeued: Option<T> = self.buf[self.ptr].take();
        self.ptr = (self.ptr + 1) % self.cap;
        self.len -= 1;

        dequeued
    }

    // Get a reference to the front element of the queue
    pub fn front(&self) -> Option<&T> {
        self.buf[self.ptr].as_ref()
    }

    pub fn rear(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        } else {
            let last_idx: usize = (self.ptr + self.len) % self.cap;

            return self.buf[last_idx].as_ref();
        }
    }

    pub fn get_bufref(&self) -> &Vec<Option<T>> {
        &self.buf
    }
}

impl<T> PartialEq for CircularQueue<T>
where
    T: Clone + PartialEq + Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.ptr == other.ptr && self.buf == other.buf
    }
}

impl<T> Eq for CircularQueue<T> where T: Clone + PartialEq + Eq {}

// Leetcode specific solution
pub struct MyCircularQueue {
    data: Vec<i32>,
    head: usize,
    len: usize,
    cap: usize,
}

impl MyCircularQueue {
    pub fn new(k: i32) -> Self {
        let k: usize = k as usize;

        Self {
            data: vec![-1; k],
            head: 0,
            len: 0,
            cap: k,
        }
    }

    pub fn enqueue(&mut self, value: i32) -> bool {
        if self.len == self.cap {
            return false;
        }

        self.data[(self.head + self.len) % self.cap] = value;
        self.len += 1;

        true
    }

    pub fn dequeue(&mut self) -> bool {
        if self.len == 0 {
            return false;
        }

        self.data[self.head] = -1;
        self.head = (self.head + 1) % self.cap;
        self.len -= 1;

        true
    }

    pub fn front(&self) -> i32 {
        self.data[self.head]
    }

    pub fn rear(&self) -> i32 {
        self.data[(self.head + self.len - 1) % self.cap]
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == self.cap
    }

    pub fn get_buffer(&self) -> Vec<i32> {
        self.data.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_queue_1() {
        let mut circ_queue: MyCircularQueue = MyCircularQueue::new(3);

        // -1 -1 -1
        assert!(circ_queue.is_empty());

        // -1 -1 1
        assert!(circ_queue.enqueue(1));
        // -1 1 2
        assert!(circ_queue.enqueue(2));
        // 1 2 3
        assert!(circ_queue.enqueue(3));
        assert!(!circ_queue.enqueue(4));

        assert!(circ_queue.is_full());

        assert_eq!(circ_queue.front(), 1);
        assert_eq!(circ_queue.rear(), 3);

        assert!(circ_queue.dequeue());

        assert_eq!(circ_queue.front(), 2);
        assert_eq!(circ_queue.rear(), 3);

        assert!(circ_queue.enqueue(1));
        assert_eq!(circ_queue.front(), 2);
        assert_eq!(circ_queue.rear(), 1);

        while !circ_queue.is_empty() {
            assert!(circ_queue.dequeue());
        }

        assert_eq!(circ_queue.front(), -1);
        assert_eq!(circ_queue.rear(), -1);
    }

    #[test]
    fn test_myqueue() {
        let mut queue: CircularQueue<u128> = CircularQueue::new(3);

        assert!(queue.is_empty());

        let _ = queue.enqueue(1);
        assert_eq!(queue.get_bufref(), &vec![Some(1), None, None]);

        let _ = queue.enqueue(2);
        let _ = queue.enqueue(3);
        assert_eq!(queue.get_bufref(), &vec![Some(1), Some(2), Some(3)]);
        assert_eq!(queue.enqueue(4), Err(QueueError::QueueFull));

        let _ = queue.dequeue();
        assert_eq!(queue.get_bufref(), &vec![None, Some(2), Some(3)]);
        let _ = queue.dequeue();
        assert_eq!(queue.get_bufref(), &vec![None, None, Some(3)]);

        let _ = queue.enqueue(1);
        assert_eq!(queue.get_bufref(), &vec![Some(1), None, Some(3)]);

        let _ = queue.enqueue(5);
        assert_eq!(queue.get_bufref(), &vec![Some(1), Some(5), Some(3)]);

        assert!(queue.is_full());
        assert_eq!(queue.enqueue(10), Err(QueueError::QueueFull));
    }

    #[test]
    fn test_myqueue_eq() {
        let mut queue_1: CircularQueue<i32> = CircularQueue::new(3);
        let mut queue_2: CircularQueue<i32> = CircularQueue::new(3);

        assert_eq!(queue_1, queue_2);

        let _ = queue_1.enqueue(1);
        let _ = queue_2.enqueue(1);
        assert_eq!(queue_1, queue_2);

        let _ = queue_1.enqueue(2);
        let _ = queue_2.enqueue(3);
        assert_ne!(queue_1, queue_2);

        let mut queue_3: CircularQueue<i32> = CircularQueue::new(3);
        let _ = queue_3.enqueue(1);
        let _ = queue_3.enqueue(2);
        assert_eq!(queue_1, queue_3);
    }
}
