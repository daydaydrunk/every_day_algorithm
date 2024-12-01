use rand::Rng;
use std::cmp::Ordering;

const MAX_LEVEL: usize = 16;

#[derive(Debug)]
struct Node<T> {
    key: T,
    forward: Vec<Option<Box<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(key: T, level: usize) -> Self {
        Node {
            key,
            forward: vec![None; level + 1],
        }
    }
}

#[derive(Debug)]
pub struct SkipList<T> {
    head: Box<Node<T>>,
    level: usize,
    length: usize,
}

impl<T: Ord + Clone> SkipList<T> {
    pub fn new() -> Self {
        SkipList {
            head: Box::new(Node {
                key: unsafe { std::mem::zeroed() },
                forward: vec![None; MAX_LEVEL],
            }),
            level: 0,
            length: 0,
        }
    }

    fn random_level() -> usize {
        let mut level = 0;
        let mut rng = rand::thread_rng();
        while rng.gen::<bool>() && level < MAX_LEVEL - 1 {
            level += 1;
        }
        level
    }

    pub fn insert(&mut self, key: T) {
        let mut update = vec![None; MAX_LEVEL];
        let mut current = &mut self.head;

        for i in (0..=self.level).rev() {
            while let Some(ref next) = current.forward[i] {
                match key.cmp(&next.key) {
                    Ordering::Greater => {
                        current = unsafe { &mut *(next as *const _ as *mut Node<T>) }
                    }
                    _ => break,
                }
            }
            update[i] = Some(current);
        }

        let level = Self::random_level();
        if level > self.level {
            for i in (self.level + 1)..=level {
                update[i] = Some(&mut self.head);
            }
            self.level = level;
        }

        let new_node = Box::new(Node::new(key, level));
        for i in 0..=level {
            if let Some(node) = &mut update[i] {
                let temp = node.forward[i].take();
                new_node.forward[i] = temp;
                node.forward[i] = Some(new_node.clone());
            }
        }
        self.length += 1;
    }

    pub fn search(&self, key: &T) -> bool {
        let mut current = &self.head;

        for i in (0..=self.level).rev() {
            while let Some(ref next) = current.forward[i] {
                match key.cmp(&next.key) {
                    Ordering::Less => break,
                    Ordering::Equal => return true,
                    Ordering::Greater => current = next,
                }
            }
        }
        false
    }

    pub fn delete(&mut self, key: &T) -> bool {
        let mut update = vec![None; MAX_LEVEL];
        let mut current = &mut self.head;
        let mut found = false;

        for i in (0..=self.level).rev() {
            while let Some(ref next) = current.forward[i] {
                match key.cmp(&next.key) {
                    Ordering::Less => break,
                    Ordering::Equal => {
                        found = true;
                        break;
                    }
                    Ordering::Greater => {
                        current = unsafe { &mut *(next as *const _ as *mut Node<T>) }
                    }
                }
            }
            update[i] = Some(current);
        }

        if found {
            if let Some(node_to_delete) = update[0].as_mut().and_then(|n| n.forward[0].take()) {
                for i in 0..=self.level {
                    if let Some(ref mut prev) = update[i] {
                        if let Some(ref next) = prev.forward[i] {
                            if next.key == node_to_delete.key {
                                prev.forward[i] = node_to_delete.forward[i].clone();
                            }
                        }
                    }
                }

                while self.level > 0 && self.head.forward[self.level].is_none() {
                    self.level -= 1;
                }
                self.length -= 1;
            }
        }
        found
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_list() {
        let mut list = SkipList::new();
        list.insert(3);
        list.insert(6);
        list.insert(7);
        list.insert(9);
        list.insert(12);
        list.insert(19);
        list.insert(17);

        assert!(list.search(&6));
        assert!(!list.search(&4));
        assert!(list.delete(&6));
        assert!(!list.search(&6));
        assert_eq!(list.len(), 6);
    }
}
