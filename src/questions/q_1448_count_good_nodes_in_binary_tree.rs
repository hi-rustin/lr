use std::{cell::RefCell, rc::Rc};

use crate::utils::TreeNode;

pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, max_value: i32) -> i32 {
        if root.is_none() {
            return 0;
        }

        let node_value = root.as_ref().unwrap().borrow().val;
        let res = if node_value >= max_value { 1 } else { 0 };
        let max_value = i32::max(max_value, node_value);
        res + dfs(root.as_ref().unwrap().borrow().left.clone(), max_value)
            + dfs(root.as_ref().unwrap().borrow().right.clone(), max_value)
    }
    dfs(root.clone(), root.as_ref().unwrap().borrow().val)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test_good_nodes() {
        assert_eq!(good_nodes(tree!(1, 2, 3)), 3);
        assert_eq!(good_nodes(tree!(3, 1, 4, 3, None, 1, 5)), 4);
    }
}
