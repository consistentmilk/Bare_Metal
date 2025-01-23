pub type ListLink = Option<Box<ListNode>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: ListLink,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

#[macro_export]
macro_rules! list {
    () => {
        None
    };

    ($e: expr) => {
        ListLink::link($e, None)
    };

    ($e: expr, $($tail: tt)*) => {
        ListLink::link($e, list!($($tail)*))
    };
}

pub trait ListMaker {
    fn link(val: i32, next: ListLink) -> ListLink {
        Some(Box::new(ListNode { val, next }))
    }
}

impl ListMaker for ListLink {}

pub fn list_printer(head: Option<Box<ListNode>>) {
    let mut ptr: Option<Box<ListNode>> = head;
    let mut linked_list_str: String = String::new();

    let mut node_val_container: Vec<i32> = vec![];

    while let Some(node) = ptr {
        node_val_container.push(node.val);

        ptr = node.next;
    }

    let n: usize = node_val_container.len();

    for i in 0..n - 1 {
        linked_list_str.push_str(format!("{} -> ", node_val_container[i]).as_str());
    }

    linked_list_str.push_str(format!("{}", node_val_container[n - 1]).as_str());

    println!("{}", linked_list_str);
}
