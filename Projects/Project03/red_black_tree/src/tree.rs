mod tree_color;
mod tree_error;
mod tree_node;

pub use tree_color::TreeColor;
pub use tree_error::TreeError;
pub use tree_node::TreeNode;

use core::ffi::c_void;
use std::{
    mem,
    ptr::{self},
};

use crate::{BOLD, CharContainer, RED, RESET};

pub struct RedBlackTree {
    root: *mut TreeNode,
}

impl Default for RedBlackTree {
    fn default() -> Self {
        Self {
            root: ptr::null_mut(),
        }
    }
}

impl RedBlackTree {
    fn create_node(key: u64, value: CharContainer) -> *mut TreeNode {
        unsafe {
            let size = mem::size_of::<TreeNode>();
            let data = libc::malloc(size) as *mut TreeNode;
            if !data.is_null() {
                data.write(TreeNode::new(key, value));
            }
            data
        }
    }

    unsafe fn free_node(node: *mut TreeNode) {
        if node.is_null() {
            return;
        }

        unsafe {
            Self::free_node((*node).left);
            Self::free_node((*node).right);
            ptr::drop_in_place(node);
            libc::free(node as *mut c_void);
        }
    }

    /// Algorithm translated to Rust from "Introduction To Algorithms Third Edition"
    /// chapter 13.3 "Red-Black Trees - Insertion", page 315, 316
    pub fn insert(&mut self, key: u64, val: CharContainer) -> Result<(), TreeError> {
        let node = RedBlackTree::create_node(key, val);
        if node.is_null() {
            return Err(TreeError::AllocationFailed);
        }

        unsafe {
            let mut parent = ptr::null_mut();
            let mut current = self.root;

            while !current.is_null() {
                parent = current;
                if (*node).key < (*current).key {
                    current = (*current).left;
                } else if (*node).key > (*current).key {
                    current = (*current).right;
                } else {
                    (*current).value = (*node).value.clone();
                    Self::free_node(node);
                    return Ok(());
                }
            }

            (*node).parent = parent;
            if parent.is_null() {
                self.root = node;
            } else if (*node).key < (*parent).key {
                (*parent).left = node;
            } else {
                (*parent).right = node;
            }

            self.insert_fixup(node);
        }
        Ok(())
    }

    unsafe fn insert_fixup(&mut self, mut current: *mut TreeNode) {
        unsafe {
            while !(*current).parent.is_null() && (*(*current).parent).color == TreeColor::Red {
                let parent = (*current).parent;
                let grandparent = (*parent).parent;
                if parent == (*grandparent).left {
                    current = self.fix_insert_left_case(current);
                } else {
                    current = self.fix_insert_right_case(current);
                }
            }
            (*self.root).color = TreeColor::Black;
        }
    }

    unsafe fn fix_insert_left_case(&mut self, mut current: *mut TreeNode) -> *mut TreeNode {
        unsafe {
            let parent = (*current).parent;
            let grandparent = (*parent).parent;
            let uncle = (*grandparent).right;

            if !uncle.is_null() && (*uncle).color == TreeColor::Red {
                (*parent).color = TreeColor::Black;
                (*uncle).color = TreeColor::Black;
                (*grandparent).color = TreeColor::Red;
                return grandparent;
            }

            if current == (*parent).right {
                current = parent;
                self.rotate_left(current);
            }

            let parent = (*current).parent;
            (*parent).color = TreeColor::Black;
            (*grandparent).color = TreeColor::Red;
            self.rotate_right(grandparent);

            current
        }
    }

    unsafe fn fix_insert_right_case(&mut self, mut current: *mut TreeNode) -> *mut TreeNode {
        unsafe {
            let parent = (*current).parent;
            let grandparent = (*parent).parent;
            let uncle = (*grandparent).left;

            if !uncle.is_null() && (*uncle).color == TreeColor::Red {
                (*parent).color = TreeColor::Black;
                (*uncle).color = TreeColor::Black;
                (*grandparent).color = TreeColor::Red;
                return grandparent;
            }

            if current == (*parent).left {
                current = parent;
                self.rotate_right(current);
            }

            let parent = (*current).parent;
            (*parent).color = TreeColor::Black;
            (*grandparent).color = TreeColor::Red;
            self.rotate_left(grandparent);

            current
        }
    }

    /// Algorithm translated to Rust from "Introduction To Algorithms Third Edition"
    /// chapter 13.2 "Red-Black Trees - Rotations", page 313, 314
    unsafe fn rotate_left(&mut self, node: *mut TreeNode) {
        unsafe {
            let right_child = (*node).right;
            (*node).right = (*right_child).left;

            if !(*right_child).left.is_null() {
                (*(*right_child).left).parent = node;
            }
            (*right_child).parent = (*node).parent;

            if (*node).parent.is_null() {
                self.root = right_child;
            } else if node == (*(*node).parent).left {
                (*(*node).parent).left = right_child;
            } else {
                (*(*node).parent).right = right_child;
            }

            (*right_child).left = node;
            (*node).parent = right_child;
        }
    }

    unsafe fn rotate_right(&mut self, node: *mut TreeNode) {
        unsafe {
            let left_child = (*node).left;
            (*node).left = (*left_child).right;

            if !(*left_child).right.is_null() {
                (*(*left_child).right).parent = node;
            }
            (*left_child).parent = (*node).parent;

            if (*node).parent.is_null() {
                self.root = left_child;
            } else if node == (*(*node).parent).right {
                (*(*node).parent).right = left_child;
            } else {
                (*(*node).parent).left = left_child;
            }

            (*left_child).right = node;
            (*node).parent = left_child;
        }
    }

    pub fn get(&self, key: u64) -> Option<&str> {
        unsafe {
            let node = self.find(key);
            if node.is_null() {
                None
            } else {
                let content: &str = (&(*node).value).into();
                Some(content)
            }
        }
    }

    /// Algorithm translated to Rust from "Introduction To Algorithms Third Edition"
    /// chapter 13.2 "Red-Black Trees - Deletion", page 324
    pub fn remove(&mut self, key: u64) -> Result<(), TreeError> {
        unsafe {
            let node = self.find(key);
            if node.is_null() {
                return Err(TreeError::NotFound);
            }
            self.delete_node_internal(node);
        }
        Ok(())
    }

    unsafe fn delete_node_internal(&mut self, z: *mut TreeNode) {
        unsafe {
            let mut y = z;
            let mut y_original_color = (*y).color;
            let x: *mut TreeNode;
            let x_parent: *mut TreeNode;

            if (*z).left.is_null() {
                x = (*z).right;
                let actual_parent = (*z).parent;

                self.transplant(z, (*z).right);

                x_parent = if !x.is_null() {
                    (*x).parent
                } else {
                    actual_parent
                };
            } else if (*z).right.is_null() {
                x = (*z).left;
                let actual_parent = (*z).parent;

                self.transplant(z, (*z).left);

                x_parent = if !x.is_null() {
                    (*x).parent
                } else {
                    actual_parent
                };
            } else {
                y = self.minimum((*z).right);
                y_original_color = (*y).color;
                x = (*y).right;

                if (*y).parent == z {
                    x_parent = y;
                } else {
                    x_parent = (*y).parent;
                    self.transplant(y, (*y).right);
                    (*y).right = (*z).right;
                    (*(*y).right).parent = y;
                }

                self.transplant(z, y);
                (*y).left = (*z).left;
                (*(*y).left).parent = y;
                (*y).color = (*z).color;
            }

            if y_original_color == TreeColor::Black {
                self.delete_fixup(x, x_parent);
            }

            ptr::drop_in_place(&mut (*z).value);
            libc::free(z as *mut c_void);
        }
    }

    /// Algorithm translated to Rust from "Introduction To Algorithms Third Edition"
    /// chapter 13.2 "Red-Black Trees - Deletion", page 323
    unsafe fn transplant(&mut self, target: *mut TreeNode, replacement: *mut TreeNode) {
        unsafe {
            if (*target).parent.is_null() {
                self.root = replacement;
            } else if target == (*(*target).parent).left {
                (*(*target).parent).left = replacement;
            } else {
                (*(*target).parent).right = replacement;
            }
            if !replacement.is_null() {
                (*replacement).parent = (*target).parent;
            }
        }
    }

    unsafe fn minimum(&self, mut node: *mut TreeNode) -> *mut TreeNode {
        unsafe {
            while !(*node).left.is_null() {
                node = (*node).left;
            }
            node
        }
    }

    unsafe fn find(&self, key: u64) -> *mut TreeNode {
        let mut node = self.root;
        unsafe {
            while !node.is_null() && key != (*node).key {
                if key < (*node).key {
                    node = (*node).left;
                } else {
                    node = (*node).right;
                }
            }
            node
        }
    }

    pub fn contains(&self, key: u64) -> bool {
        unsafe { !self.find(key).is_null() }
    }

    /// Algorithm translated to Rust from "Introduction To Algorithms Third Edition"
    /// chapter 13.2 "Red-Black Trees - Deletion", page 326
    unsafe fn delete_fixup(&mut self, mut current: *mut TreeNode, mut parent: *mut TreeNode) {
        unsafe {
            while current != self.root
                && (current.is_null() || (*current).color == TreeColor::Black)
            {
                if current == (*parent).left {
                    current = self.fix_delete_left(&mut parent);
                } else {
                    current = self.fix_delete_right(&mut parent);
                }
            }
            if !current.is_null() {
                (*current).color = TreeColor::Black;
            }
        }
    }

    unsafe fn fix_delete_left(&mut self, parent: &mut *mut TreeNode) -> *mut TreeNode {
        unsafe {
            let mut sibling = (*(*parent)).right;

            if sibling.is_null() {
                return self.root;
            }

            if (*sibling).color == TreeColor::Red {
                (*sibling).color = TreeColor::Black;
                (*(*parent)).color = TreeColor::Red;

                self.rotate_left(*parent);

                sibling = (*(*parent)).right;

                if sibling.is_null() {
                    return self.root;
                }
            }

            let sibling_left_black =
                (*sibling).left.is_null() || (*(*sibling).left).color == TreeColor::Black;

            let sibling_right_black =
                (*sibling).right.is_null() || (*(*sibling).right).color == TreeColor::Black;

            if sibling_left_black && sibling_right_black {
                (*sibling).color = TreeColor::Red;

                let current = *parent;
                *parent = (*(*parent)).parent;

                return current;
            }

            if sibling_right_black {
                if !(*sibling).left.is_null() {
                    (*(*sibling).left).color = TreeColor::Black;
                }

                (*sibling).color = TreeColor::Red;
                self.rotate_right(sibling);

                sibling = (*(*parent)).right;
                if sibling.is_null() {
                    return self.root;
                }
            }

            (*sibling).color = (*(*parent)).color;
            (*(*parent)).color = TreeColor::Black;

            if !(*sibling).right.is_null() {
                (*(*sibling).right).color = TreeColor::Black;
            }

            self.rotate_left(*parent);
            self.root
        }
    }

    unsafe fn fix_delete_right(&mut self, parent: &mut *mut TreeNode) -> *mut TreeNode {
        unsafe {
            let mut sibling = (*(*parent)).left;

            if sibling.is_null() {
                return self.root;
            }

            if (*sibling).color == TreeColor::Red {
                (*sibling).color = TreeColor::Black;
                (*(*parent)).color = TreeColor::Red;

                self.rotate_right(*parent);

                sibling = (*(*parent)).left;
                if sibling.is_null() {
                    return self.root;
                }
            }

            let sibling_left_black =
                (*sibling).left.is_null() || (*(*sibling).left).color == TreeColor::Black;

            let sibling_right_black =
                (*sibling).right.is_null() || (*(*sibling).right).color == TreeColor::Black;

            if sibling_left_black && sibling_right_black {
                (*sibling).color = TreeColor::Red;

                let current = *parent;
                *parent = (*(*parent)).parent;
                return current;
            }

            if sibling_left_black {
                if !(*sibling).right.is_null() {
                    (*(*sibling).right).color = TreeColor::Black;
                }

                (*sibling).color = TreeColor::Red;
                self.rotate_left(sibling);

                sibling = (*(*parent)).left;
                if sibling.is_null() {
                    return self.root;
                }
            }

            (*sibling).color = (*(*parent)).color;
            (*(*parent)).color = TreeColor::Black;

            if !(*sibling).left.is_null() {
                (*(*sibling).left).color = TreeColor::Black;
            }

            self.rotate_right(*parent);
            self.root
        }
    }

    pub fn print_structure(&self) {
        unsafe {
            Self::print_node_internal(self.root, 0);
        }
    }

    /// Idea for the printing algorithm taken from:
    /// https://www.geeksforgeeks.org/dsa/print-binary-tree-2-dimensions/
    unsafe fn print_node_internal(node: *mut TreeNode, level: usize) {
        if node.is_null() {
            return;
        }

        unsafe {
            Self::print_node_internal((*node).right, level + 1);
            let indent = "    ".repeat(level);
            let color_code = match (*node).color {
                TreeColor::Red => RED,
                TreeColor::Black => BOLD,
            };

            let content: &str = (&(*node).value).into();

            println!(
                "{indent}{color_code}[{}] {content}{RESET} ({})",
                (*node).key,
                (*node).color,
            );

            Self::print_node_internal((*node).left, level + 1);
        }
    }
}

impl Drop for RedBlackTree {
    fn drop(&mut self) {
        unsafe {
            Self::free_node(self.root);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CharContainer;

    fn create_value(s: &str) -> CharContainer {
        CharContainer::new(s).expect("Allocation failed")
    }

    #[test]
    fn test_new_tree_is_empty() {
        let tree = RedBlackTree::default();
        assert!(tree.root.is_null());
    }

    #[test]
    fn test_insert_root() {
        let mut tree = RedBlackTree::default();
        let value = create_value("One");
        assert!(tree.insert(1, value).is_ok());

        assert!(tree.contains(1));
        assert_eq!(tree.get(1), Some("One"));

        unsafe {
            assert!(!tree.root.is_null());
            assert_eq!((*tree.root).color, TreeColor::Black);
        }
    }

    #[test]
    fn test_insert_duplicate_updates_value() {
        let mut tree = RedBlackTree::default();
        tree.insert(1, create_value("One")).unwrap();
        tree.insert(1, create_value("Uno")).unwrap();

        assert_eq!(tree.get(1), Some("Uno"));
    }

    #[test]
    fn test_insert_multiple_balanced() {
        // Inserting 1, 2, 3 should trigger rotation
        // 2 should become root (Black), 1 and 3 children (Red)
        let mut tree = RedBlackTree::default();
        tree.insert(1, create_value("One")).unwrap();
        tree.insert(2, create_value("Two")).unwrap();
        tree.insert(3, create_value("Three")).unwrap();

        assert!(tree.contains(1));
        assert!(tree.contains(2));
        assert!(tree.contains(3));

        unsafe {
            let root = tree.root;
            assert!(!root.is_null());
            assert_eq!((*root).key, 2);
            assert_eq!((*root).color, TreeColor::Black);

            let left = (*root).left;
            let right = (*root).right;

            assert!(!left.is_null());
            assert_eq!((*left).key, 1);
            assert_eq!((*left).color, TreeColor::Red);

            assert!(!right.is_null());
            assert_eq!((*right).key, 3);
            assert_eq!((*right).color, TreeColor::Red);
        }
    }

    #[test]
    fn test_get_not_found() {
        let mut tree = RedBlackTree::default();
        tree.insert(10, create_value("Ten")).unwrap();
        assert_eq!(tree.get(11), None);
    }

    #[test]
    fn test_remove_leaf() {
        let mut tree = RedBlackTree::default();
        tree.insert(10, create_value("Ten")).unwrap();
        tree.insert(5, create_value("Five")).unwrap(); // Leaf

        assert!(tree.contains(5));
        assert!(tree.remove(5).is_ok());
        assert!(!tree.contains(5));
        assert!(tree.contains(10));
    }

    #[test]
    fn test_remove_root() {
        let mut tree = RedBlackTree::default();
        tree.insert(10, create_value("Ten")).unwrap();

        assert!(tree.remove(10).is_ok());
        assert!(!tree.contains(10));
        assert!(tree.root.is_null());
    }

    #[test]
    fn test_remove_not_found() {
        let mut tree = RedBlackTree::default();
        tree.insert(10, create_value("Ten")).unwrap();

        let result = tree.remove(99);
        assert!(matches!(result, Err(TreeError::NotFound)));
    }

    #[test]
    fn test_complex_scenario_1_to_10() {
        let mut tree = RedBlackTree::default();

        for i in 1..=10 {
            tree.insert(i, create_value(&format!("{i}"))).unwrap();
        }

        for i in 1..=10 {
            assert!(tree.contains(i));
        }

        for i in (1..=10).step_by(2) {
            assert!(tree.remove(i).is_ok());
        }

        for i in 1..=10 {
            if i % 2 != 0 {
                assert!(!tree.contains(i), "Tree should not contain {i}");
            } else {
                assert!(tree.contains(i), "Tree should contain {i}");
            }
        }
    }
}
