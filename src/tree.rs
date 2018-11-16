use std::cmp::PartialOrd;
use std::fmt::Display;

type TreeNode<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug)]
struct Node<K, V: Display> {
    left_node: TreeNode<K, V>,
    right_node: TreeNode<K, V>,
    key: K,
    value: V,
}

trait BinaryTree<K, V> {
    fn pre_order(&self);
    fn in_order(&self);
    fn pos_order(&self);
}

trait BinarySearchTree<K: PartialOrd, V>: BinaryTree<K, V> {
    fn insert(&mut self, key: K, value: V);
}

impl<K, V: Display> Node<K, V> {
    fn new(key: K, value: V) -> Self {
        Node {
            left_node: None,
            right_node: None,
            key: key,
            value: value,
        }
    }
}

impl<K: PartialOrd, V: Display> BinarySearchTree<K, V> for Node<K, V> {
    fn insert(&mut self, key: K, value: V) {
        if self.key < key {
            if let Some(ref mut right) = self.right_node {
                right.insert(key, value);
            } else {
                self.right_node = Some(Box::new(Node::new(key, value)));
            }
        } else {
            if let Some(ref mut left) = self.left_node {
                left.insert(key, value);
            } else {
                self.left_node = Some(Box::new(Node::new(key, value)));
            }
        }
    }
}

impl<K, V: Display> BinaryTree<K, V> for Node<K, V> {
    fn pre_order(&self) {
        println!("{}", self.value);
        if let Some(ref left) = self.left_node {
            left.pre_order();
        }
        if let Some(ref right) = self.right_node {
            right.pre_order();
        }
    }
    fn in_order(&self) {
        if let Some(ref left) = self.left_node {
            left.in_order();
        }
        println!("{}", self.value);
        if let Some(ref right) = self.right_node {
            right.in_order();
        }
    }
    fn pos_order(&self) {
        if let Some(ref left) = self.left_node {
            left.pos_order();
        }
        if let Some(ref right) = self.right_node {
            right.pos_order();
        }
        println!("{}", self.value);
    }
}

type BST<K, V> = Node<K, V>;

#[test]
fn bst_test() {
    let mut root = BST::<i32, i32>::new(3, 4);
    root.insert(2, 3);
    root.insert(4, 6);
    root.insert(5, 5);
    root.insert(6, 6);
    root.insert(1, 8);
    if let Some(ref left) = root.left_node {
        assert_eq!(left.value, 3);
    }
    if let Some(ref right) = root.right_node {
        assert_eq!(right.value, 6);
        if let Some(ref right) = right.right_node {
            assert_eq!(right.value, 5);
        }
    }
    println!("Pre Order traversal");
    root.pre_order();
    println!("In Order traversal");
    root.in_order();
    println!("Pos Order traversal");
    root.pos_order();
}
