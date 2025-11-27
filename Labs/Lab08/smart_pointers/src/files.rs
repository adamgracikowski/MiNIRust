#![allow(dead_code)]

use std::{
    cell::{LazyCell, OnceCell},
    path::Path,
    rc::Rc,
};

pub struct CachedFile {
    cache: OnceCell<String>,
}

impl Default for CachedFile {
    fn default() -> Self {
        Self::new()
    }
}

impl CachedFile {
    pub fn new() -> Self {
        CachedFile {
            cache: OnceCell::new(),
        }
    }

    pub fn get(&self, path: &Path) -> &str {
        self.cache
            .get_or_init(|| std::fs::read_to_string(path).expect("Failed to read the file"))
    }

    pub fn try_get(&self) -> Option<&str> {
        self.cache.get().map(|s| s.as_str())
    }
}

type Initializer = Box<dyn FnOnce() -> String>;

#[derive(Clone)]
pub struct SharedFile {
    content: Rc<LazyCell<String, Initializer>>,
}

impl SharedFile {
    pub fn new(path: &Path) -> Self {
        let path = path.to_path_buf();
        let initializer: Initializer =
            Box::new(move || std::fs::read_to_string(&path).expect("Failed to read the file"));

        SharedFile {
            content: Rc::new(LazyCell::new(initializer)),
        }
    }

    pub fn get(&self) -> &str {
        &self.content
    }
}
