use std::ptr;

use crate::CharContainer;

use super::TreeColor;

pub struct TreeNode {
    pub key: u64,
    pub value: CharContainer,
    pub color: TreeColor,

    pub left: *mut TreeNode,
    pub right: *mut TreeNode,
    pub parent: *mut TreeNode,
}

impl TreeNode {
    pub fn new(key: u64, value: CharContainer) -> Self {
        Self {
            key,
            value,
            color: TreeColor::default(),
            left: ptr::null_mut(),
            right: ptr::null_mut(),
            parent: ptr::null_mut(),
        }
    }
}
