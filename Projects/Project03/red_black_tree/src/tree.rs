mod tree_color;
mod tree_node;

pub use tree_color::TreeColor;
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

#[derive(Debug)]
pub enum DictionaryError {
    AllocationFailed,
    NotFound,
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
    pub fn insert(&mut self, key: u64, val: CharContainer) -> Result<(), DictionaryError> {
        let node = RedBlackTree::create_node(key, val);
        if node.is_null() {
            return Err(DictionaryError::AllocationFailed);
        }

        unsafe {
            let mut y = ptr::null_mut();
            let mut x = self.root;

            while !x.is_null() {
                y = x;
                if (*node).key < (*x).key {
                    x = (*x).left;
                } else if (*node).key > (*x).key {
                    x = (*x).right;
                } else {
                    (*x).value = (*node).value.clone();
                    Self::free_node(node);
                    return Ok(());
                }
            }

            (*node).parent = y;
            if y.is_null() {
                self.root = node;
            } else if (*node).key < (*y).key {
                (*y).left = node;
            } else {
                (*y).right = node;
            }

            self.insert_fixup(node);
        }
        Ok(())
    }

    unsafe fn insert_fixup(&mut self, mut z: *mut TreeNode) {
        unsafe {
            while !(*z).parent.is_null() && (*(*z).parent).color == TreeColor::Red {
                if (*z).parent == (*(*(*z).parent).parent).left {
                    z = self.fix_insert_left_case(z);
                } else {
                    z = self.fix_insert_right_case(z);
                }
            }
            (*self.root).color = TreeColor::Black;
        }
    }

    unsafe fn fix_insert_left_case(&mut self, mut z: *mut TreeNode) -> *mut TreeNode {
        unsafe {
            let gp = (*(*z).parent).parent;
            let y = (*gp).right; // uncle

            if !y.is_null() && (*y).color == TreeColor::Red {
                (*(*z).parent).color = TreeColor::Black;
                (*y).color = TreeColor::Black;
                (*gp).color = TreeColor::Red;
                return gp;
            }

            if z == (*(*z).parent).right {
                z = (*z).parent;
                self.rotate_left(z);
            }
            (*(*z).parent).color = TreeColor::Black;
            (*gp).color = TreeColor::Red;
            self.rotate_right(gp);
            z
        }
    }

    unsafe fn fix_insert_right_case(&mut self, mut z: *mut TreeNode) -> *mut TreeNode {
        unsafe {
            let gp = (*(*z).parent).parent;
            let y = (*gp).left; // uncle

            if !y.is_null() && (*y).color == TreeColor::Red {
                (*(*z).parent).color = TreeColor::Black;
                (*y).color = TreeColor::Black;
                (*gp).color = TreeColor::Red;
                return gp;
            }

            if z == (*(*z).parent).left {
                z = (*z).parent;
                self.rotate_right(z);
            }
            (*(*z).parent).color = TreeColor::Black;
            (*gp).color = TreeColor::Red;
            self.rotate_left(gp);
            z
        }
    }

    /// Algorithm translated to Rust from "Introduction To Algorithms Third Edition"
    /// chapter 13.2 "Red-Black Trees - Rotations", page 313, 314
    unsafe fn rotate_left(&mut self, x: *mut TreeNode) {
        unsafe {
            let y = (*x).right;
            (*x).right = (*y).left;

            if !(*y).left.is_null() {
                (*(*y).left).parent = x;
            }
            (*y).parent = (*x).parent;

            if (*x).parent.is_null() {
                self.root = y;
            } else if x == (*(*x).parent).left {
                (*(*x).parent).left = y;
            } else {
                (*(*x).parent).right = y;
            }

            (*y).left = x;
            (*x).parent = y;
        }
    }

    unsafe fn rotate_right(&mut self, x: *mut TreeNode) {
        unsafe {
            let y = (*x).left;
            (*x).left = (*y).right;

            if !(*y).right.is_null() {
                (*(*y).right).parent = x;
            }
            (*y).parent = (*x).parent;

            if (*x).parent.is_null() {
                self.root = y;
            } else if x == (*(*x).parent).right {
                (*(*x).parent).right = y;
            } else {
                (*(*x).parent).left = y;
            }

            (*y).right = x;
            (*x).parent = y;
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
    pub fn remove(&mut self, key: u64) -> Result<(), DictionaryError> {
        unsafe {
            let z = self.find(key);
            if z.is_null() {
                return Err(DictionaryError::NotFound);
            }
            self.delete_node_internal(z);
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
    unsafe fn transplant(&mut self, u: *mut TreeNode, v: *mut TreeNode) {
        unsafe {
            if (*u).parent.is_null() {
                self.root = v;
            } else if u == (*(*u).parent).left {
                (*(*u).parent).left = v;
            } else {
                (*(*u).parent).right = v;
            }
            if !v.is_null() {
                (*v).parent = (*u).parent;
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
    unsafe fn delete_fixup(&mut self, mut x: *mut TreeNode, mut p: *mut TreeNode) {
        unsafe {
            while x != self.root && (x.is_null() || (*x).color == TreeColor::Black) {
                if x == (*p).left {
                    x = self.fix_delete_left(&mut p);
                } else {
                    x = self.fix_delete_right(&mut p);
                }
            }
            if !x.is_null() {
                (*x).color = TreeColor::Black;
            }
        }
    }

    unsafe fn fix_delete_left(&mut self, p: &mut *mut TreeNode) -> *mut TreeNode {
        unsafe {
            let mut w = (*(*p)).right;

            if w.is_null() {
                return self.root;
            }

            if (*w).color == TreeColor::Red {
                (*w).color = TreeColor::Black;
                (*(*p)).color = TreeColor::Red;
                self.rotate_left(*p);
                w = (*(*p)).right;
                if w.is_null() {
                    return self.root;
                }
            }

            if ((*w).left.is_null() || (*(*w).left).color == TreeColor::Black)
                && ((*w).right.is_null() || (*(*w).right).color == TreeColor::Black)
            {
                (*w).color = TreeColor::Red;
                let new_x = *p;
                *p = (*(*p)).parent;
                new_x
            } else {
                if (*w).right.is_null() || (*(*w).right).color == TreeColor::Black {
                    if !(*w).left.is_null() {
                        (*(*w).left).color = TreeColor::Black;
                    }
                    (*w).color = TreeColor::Red;
                    self.rotate_right(w);
                    w = (*(*p)).right;
                    if w.is_null() {
                        return self.root;
                    }
                }

                (*w).color = (*(*p)).color;
                (*(*p)).color = TreeColor::Black;
                if !(*w).right.is_null() {
                    (*(*w).right).color = TreeColor::Black;
                }
                self.rotate_left(*p);
                self.root
            }
        }
    }

    unsafe fn fix_delete_right(&mut self, p: &mut *mut TreeNode) -> *mut TreeNode {
        unsafe {
            let mut w = (*(*p)).left;

            if w.is_null() {
                return self.root;
            }

            if (*w).color == TreeColor::Red {
                (*w).color = TreeColor::Black;
                (*(*p)).color = TreeColor::Red;
                self.rotate_right(*p);
                w = (*(*p)).left;
                if w.is_null() {
                    return self.root;
                }
            }

            if ((*w).right.is_null() || (*(*w).right).color == TreeColor::Black)
                && ((*w).left.is_null() || (*(*w).left).color == TreeColor::Black)
            {
                (*w).color = TreeColor::Red;
                let new_x = *p;
                *p = (*(*p)).parent;
                new_x
            } else {
                if (*w).left.is_null() || (*(*w).left).color == TreeColor::Black {
                    if !(*w).right.is_null() {
                        (*(*w).right).color = TreeColor::Black;
                    }
                    (*w).color = TreeColor::Red;
                    self.rotate_left(w);
                    w = (*(*p)).left;
                    if w.is_null() {
                        return self.root;
                    }
                }

                (*w).color = (*(*p)).color;
                (*(*p)).color = TreeColor::Black;
                if !(*w).left.is_null() {
                    (*(*w).left).color = TreeColor::Black;
                }
                self.rotate_right(*p);
                self.root
            }
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
