use std::cell::RefCell;
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
    pub fn insert_into_max_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // If root is None or val is greater than root, make val the new root
        if root.is_none() || val > root.as_ref().unwrap().borrow().val {
            let mut new_node = TreeNode::new(val);
            new_node.left = root;
            return Some(Rc::new(RefCell::new(new_node)));
        }

        // Otherwise, recursively insert into right subtree
        let root_ref = root.as_ref().unwrap();
        let mut root_borrowed = root_ref.borrow_mut();
        root_borrowed.right = Self::insert_into_max_tree(root_borrowed.right.take(), val);
        drop(root_borrowed);

        root
    }
}

// Test cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_into_max_tree() {
        // Helper function to create tree nodes
        fn create_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
            Some(Rc::new(RefCell::new(TreeNode::new(val))))
        }

        // Test case 1: [4,1,3,null,null,2] + val=5
        let mut root = create_node(4);
        let root_ref = root.as_ref().unwrap();
        root_ref.borrow_mut().left = create_node(1);
        root_ref.borrow_mut().right = create_node(3);
        root_ref
            .borrow_mut()
            .right
            .as_ref()
            .unwrap()
            .borrow_mut()
            .left = create_node(2);

        let result = Solution::insert_into_max_tree(root, 5);
        assert_eq!(result.as_ref().unwrap().borrow().val, 5);
        assert_eq!(
            result
                .as_ref()
                .unwrap()
                .borrow()
                .left
                .as_ref()
                .unwrap()
                .borrow()
                .val,
            4
        );
    }
}
