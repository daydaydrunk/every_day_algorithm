use std::cell::RefCell;
use std::sync::{Arc, Mutex};

type Node<T> = Option<Arc<Mutex<Box<ListNode<T>>>>>;

pub struct List<T> {
    head: Option<Box<ListNode<T>>>,
    last: Option<Box<ListNode<T>>>,
    len: u64,
}

#[derive(Debug, Clone, Default)]
pub struct ListNode<T> {
    index: u64,
    val: T,
    next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    fn new(index: u64, val: T) -> ListNode<T> {
        ListNode {
            index: index,
            val: val,
            next: None,
        }
    }
}

impl<T> Iterator for List<T> {
    type Item = T;

    fn next(&self) -> Option<Self::Item> {
        self.head
    }
}

impl<T> List<T> {
    fn new() -> List<T> {
        List {
            head: None,
            last: None,
            len: 0,
        }
    }

    fn insert(&mut self, v: T) {
        let mut new_node = Some(Box::new(ListNode::new(self.len + 1, v)));
        match self.head {
            Some(mut h) => {}
            None => {
                if let Some(mut l) = self.last {
                    l.next = new_node;
                    self.last = new_node;
                }
                self.head = Some(Box::new(new_node));
                self.last = self.head.clone();
            }
        }
    }

    fn search(&self, id: u64) {}

    fn del(&mut self, id: u64) {}
}

//EOF
