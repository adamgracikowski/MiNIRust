mod char_container;
mod macros;
mod tree;

pub use char_container::CharContainer;
pub use tree::RedBlackTree;

use core::ffi::{c_char, c_void};
use std::{mem, ptr};

const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const BOLD: &str = "\x1b[1m";

/// Creates a new, empty Red-Black Tree.
///
/// # Safety
///
/// This function allocates memory using [`libc::malloc`].
/// The caller owns the returned pointer and is responsible for eventually
/// freeing it by calling [`tree_free`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tree_create() -> *mut RedBlackTree {
    unsafe {
        let ptr = libc::malloc(mem::size_of::<RedBlackTree>()) as *mut RedBlackTree;
        if !ptr.is_null() {
            ptr.write(RedBlackTree::default());
        }
        ptr
    }
}

/// Inserts a key-value pair into the tree.
///
/// # Safety
///
/// * `tree` must be a valid, non-null pointer to an initialized `RedBlackTree`.
/// * `value` must be a valid, non-null pointer to a **null-terminated** C string.
/// * The memory pointed to by `tree` and `value` must be accessible.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tree_insert(
    tree: *mut RedBlackTree,
    key: u64,
    value: *const c_char,
) -> i32 {
    if tree.is_null() || value.is_null() {
        return -1;
    }
    unsafe {
        if let Some(data) = CharContainer::from_c_str(value) {
            match (*tree).insert(key, data) {
                Ok(_) => 0,
                Err(_) => -2,
            }
        } else {
            -2 // alloc
        }
    }
}

/// Checks if the tree contains a specific key.
///
/// # Safety
///
/// * `tree` must be a valid, non-null pointer to an initialized `RedBlackTree`.
/// * Dereferencing `tree` must be safe.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tree_contains(tree: *mut RedBlackTree, key: u64) -> i32 {
    if tree.is_null() {
        return 0;
    }

    unsafe {
        match (*tree).contains(key) {
            true => 1,
            false => 0,
        }
    }
}

/// Retrieves a value associated with a key into a provided buffer.
///
/// # Safety
///
/// * `tree` must be a valid, non-null pointer to an initialized `RedBlackTree`.
/// * `buffer` must be a valid pointer to a writable memory region of at least `buffer_size` bytes.
/// * This function performs a raw memory copy to `buffer`.
/// * Ensure `buffer_size` is large enough to hold the string plus a null terminator.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tree_get(
    tree: *mut RedBlackTree,
    key: u64,
    buffer: *mut c_char,
    buffer_size: usize,
) -> i32 {
    if tree.is_null() || buffer.is_null() {
        return -1;
    }
    unsafe {
        match (*tree).get(key) {
            Some(value) => {
                let src = value.as_ptr();
                let size = value.len();
                if size + 1 > buffer_size {
                    return -2; // alloc
                }
                ptr::copy_nonoverlapping(src, buffer as *mut u8, size);
                *buffer.add(size) = 0;
                0
            }
            None => 1,
        }
    }
}

/// Removes a key from the tree.
///
/// # Safety
///
/// * `tree` must be a valid, non-null pointer to an initialized `RedBlackTree`.
/// * The memory pointed to by `tree` must be accessible and mutable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tree_remove(tree: *mut RedBlackTree, key: u64) -> i32 {
    if tree.is_null() {
        return -1;
    }
    unsafe {
        match (*tree).remove(key) {
            Ok(_) => 0,
            Err(_) => 1,
        }
    }
}

/// Frees the memory associated with the tree.
///
/// # Safety
///
/// * `tree` must be a valid pointer previously returned by `tree_create` (or null).
/// * After calling this function, the `tree` pointer becomes invalid (dangling)
///   and must not be used again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tree_free(tree: *mut RedBlackTree) {
    if tree.is_null() {
        return;
    }
    unsafe {
        core::ptr::drop_in_place(tree);
        libc::free(tree as *mut c_void);
    }
}

/// Prints the structure of the tree to stdout for debugging purposes.
///
/// # Safety
///
/// * `tree` must be a valid, non-null pointer to an initialized `RedBlackTree`.
/// * Dereferencing `tree` must be safe.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tree_print_structure(tree: *const RedBlackTree) {
    if tree.is_null() {
        println!("(Tree is null)");
        return;
    }
    unsafe {
        (*tree).print_structure();
    }
}
