fn main() {
    let mut queue: CircularQueue<i32> = CircularQueue::new(3);

    let mut i = 1;
    while let Ok(_) = queue.enqueue(i) {
        println!("Queue -> {:?}", queue);
        i += 1;
    }

    let _ = queue.dequeue();
    while let Ok(_) = queue.enqueue(i) {
        i += 1;
    }

    while let Some(_) = queue.dequeue() {
        println!("Queue -> {:?}", queue);
    }
    println!("{:?}", queue);

    while let Ok(_) = queue.enqueue(i) {
        i += 1;
    }

    println!("{:?}", queue);
}

#[derive(Debug)]
pub struct CircularQueue<T>
where
    T: Clone,
{
    buf: Vec<Option<T>>,
    head: usize,
    ptr: usize,
    cap: usize,
}

#[derive(Debug)]
pub enum QueueError {
    QueueFull,
    QueueEmpty,
}

impl<T> CircularQueue<T>
where
    T: Clone,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            buf: vec![None; capacity],
            head: 0,
            ptr: 0,
            cap: capacity,
        }
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.ptr == 0
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.ptr == self.cap
    }

    /// Enqueue an element into the circular queue
    pub fn enqueue(&mut self, item: T) -> Result<(), QueueError> {
        if self.ptr == self.cap {
            return Err(QueueError::QueueFull);
        }

        self.buf[(self.head + self.ptr) % self.cap] = Some(item);
        self.ptr += 1;

        Ok(())
    }

    /// Dequeue an element from the circular queue
    pub fn dequeue(&mut self) -> Option<T> {
        if self.ptr == 0 {
            return None; // Queue is empty
        }

        let item: Option<T> = self.buf[self.head].take();
        self.head = (self.head + 1) % self.cap;
        self.ptr -= 1;

        item
    }

    /// Peek at the front element
    pub fn front(&self) -> Option<T> {
        self.buf[self.head].clone()
    }

    /// Peek at the rear element
    pub fn rear(&self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let last_index: usize = (self.head + self.cap - 1) % self.cap;
            self.buf[last_index].clone()
        }
    }

    pub fn get_buffer(&self) -> Vec<Option<T>> {
        self.buf.clone()
    }

    pub fn get_head_pointer(&self) -> usize {
        self.head
    }

    pub fn get_tail_pointer(&self) -> usize {
        (self.head + self.ptr) % self.cap
    }
}
