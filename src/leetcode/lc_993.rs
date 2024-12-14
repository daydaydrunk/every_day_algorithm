use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// Definition for a binary tree node
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        if root.is_none() {
            return false;
        }

        let mut queue = VecDeque::new();
        queue.push_back((root.clone(), None)); // (node, parent)

        while !queue.is_empty() {
            let level_size = queue.len();
            let mut x_parent = None;
            let mut y_parent = None;

            for _ in 0..level_size {
                if let Some((node, parent)) = queue.pop_front() {
                    let node_ref = node.as_ref().unwrap().borrow();
                    if node_ref.val == x {
                        x_parent = parent.clone();
                    }
                    if node_ref.val == y {
                        y_parent = parent.clone();
                    }
                    if node_ref.left.is_some() {
                        queue.push_back((node_ref.left.clone(), Some(node.clone())));
                    }
                    if node_ref.right.is_some() {
                        queue.push_back((node_ref.right.clone(), Some(node.clone())));
                    }
                }
            }

            if x_parent.is_some() && y_parent.is_some() {
                return x_parent != y_parent;
            }
            if x_parent.is_some() || y_parent.is_some() {
                return false;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_cousins() {
        // Helper function to create tree nodes
        fn create_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
            Some(Rc::new(RefCell::new(TreeNode::new(val))))
        }

        // Test case 1: [1,2,3,4], x = 4, y = 3
        let mut root = create_node(1);
        let root_ref = root.as_ref().unwrap();
        root_ref.borrow_mut().left = create_node(2);
        root_ref.borrow_mut().right = create_node(3);
        root_ref
            .borrow_mut()
            .left
            .as_ref()
            .unwrap()
            .borrow_mut()
            .left = create_node(4);

        assert_eq!(Solution::is_cousins(root.clone(), 4, 3), false);

        // Test case 2: [1,2,3,null,4,null,5], x = 5, y = 4
        let mut root = create_node(1);
        let root_ref = root.as_ref().unwrap();
        root_ref.borrow_mut().left = create_node(2);
        root_ref.borrow_mut().right = create_node(3);
        root_ref
            .borrow_mut()
            .left
            .as_ref()
            .unwrap()
            .borrow_mut()
            .right = create_node(4);
        root_ref
            .borrow_mut()
            .right
            .as_ref()
            .unwrap()
            .borrow_mut()
            .right = create_node(5);

        assert_eq!(Solution::is_cousins(root.clone(), 5, 4), true);

        // Test case 3: [1,2,3,null,4], x = 2, y = 3
        let mut root = create_node(1);
        let root_ref = root.as_ref().unwrap();
        root_ref.borrow_mut().left = create_node(2);
        root_ref.borrow_mut().right = create_node(3);
        root_ref
            .borrow_mut()
            .left
            .as_ref()
            .unwrap()
            .borrow_mut()
            .right = create_node(4);

        assert_eq!(Solution::is_cousins(root.clone(), 2, 3), false);
    }
}
