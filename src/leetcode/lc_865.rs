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
    pub fn subtree_with_all_deepest(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> (Option<Rc<RefCell<TreeNode>>>, i32) {
            match node {
                Some(n) => {
                    let left = dfs(n.borrow().left.clone());
                    let right = dfs(n.borrow().right.clone());

                    if left.1 > right.1 {
                        (left.0, left.1 + 1)
                    } else if left.1 < right.1 {
                        (right.0, right.1 + 1)
                    } else {
                        (Some(n.clone()), left.1 + 1)
                    }
                }
                None => (None, 0),
            }
        }

        dfs(root).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subtree_with_all_deepest() {
        // Helper function to create tree nodes
        fn create_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
            Some(Rc::new(RefCell::new(TreeNode::new(val))))
        }

        // Test case 1: [3,5,1,6,2,0,8,null,null,7,4]
        let mut root = create_node(3);
        let root_ref = root.as_ref().unwrap();
        root_ref.borrow_mut().left = create_node(5);
        root_ref.borrow_mut().right = create_node(1);
        root_ref
            .borrow_mut()
            .left
            .as_ref()
            .unwrap()
            .borrow_mut()
            .left = create_node(6);
        root_ref
            .borrow_mut()
            .left
            .as_ref()
            .unwrap()
            .borrow_mut()
            .right = create_node(2);
        root_ref
            .borrow_mut()
            .right
            .as_ref()
            .unwrap()
            .borrow_mut()
            .left = create_node(0);
        root_ref
            .borrow_mut()
            .right
            .as_ref()
            .unwrap()
            .borrow_mut()
            .right = create_node(8);
        root_ref
            .borrow_mut()
            .left
            .as_ref()
            .unwrap()
            .borrow_mut()
            .right
            .as_ref()
            .unwrap()
            .borrow_mut()
            .left = create_node(7);
        root_ref
            .borrow_mut()
            .left
            .as_ref()
            .unwrap()
            .borrow_mut()
            .right
            .as_ref()
            .unwrap()
            .borrow_mut()
            .right = create_node(4);

        let result = Solution::subtree_with_all_deepest(root.clone());
        assert_eq!(result.unwrap().borrow().val, 2);

        // Test case 2: [1]
        let root = create_node(1);
        let result = Solution::subtree_with_all_deepest(root.clone());
        assert_eq!(result.unwrap().borrow().val, 1);

        // Test case 3: [0,1,3,null,2]
        let mut root = create_node(0);
        let root_ref = root.as_ref().unwrap();
        root_ref.borrow_mut().left = create_node(1);
        root_ref.borrow_mut().right = create_node(3);
        root_ref
            .borrow_mut()
            .left
            .as_ref()
            .unwrap()
            .borrow_mut()
            .right = create_node(2);

        let result = Solution::subtree_with_all_deepest(root.clone());
        assert_eq!(result.unwrap().borrow().val, 2);
    }
}
