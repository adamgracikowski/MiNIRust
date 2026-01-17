use core::{
    ffi::{c_char, c_void},
    slice,
};
use std::ptr::{self, null_mut};

pub struct CharContainer {
    data: *mut u8,
    size: usize,
}

impl Default for CharContainer {
    fn default() -> Self {
        Self {
            data: null_mut(),
            size: Default::default(),
        }
    }
}

impl CharContainer {
    pub fn new(value: &str) -> Option<Self> {
        let size = value.len();
        unsafe {
            let data = libc::malloc(size + 1) as *mut u8;
            if data.is_null() {
                return None;
            }

            ptr::copy_nonoverlapping(value.as_ptr(), data, size);
            *data.add(size) = 0; // \0

            Some(CharContainer { data, size })
        }
    }

    pub unsafe fn from_c_str(value: *const c_char) -> Option<Self> {
        if value.is_null() {
            return None;
        }
        let mut size = 0;

        unsafe {
            // \0
            while *value.add(size) != 0 {
                size += 1;
            }

            let data = libc::malloc(size + 1) as *mut u8;
            if data.is_null() {
                return None;
            }

            ptr::copy_nonoverlapping(value as *const u8, data, size + 1);
            Some(CharContainer { data, size })
        }
    }
}

impl From<&CharContainer> for &str {
    fn from(value: &CharContainer) -> Self {
        unsafe {
            let raw = slice::from_raw_parts(value.data, value.size);
            str::from_utf8_unchecked(raw)
        }
    }
}

impl Drop for CharContainer {
    fn drop(&mut self) {
        unsafe {
            if !self.data.is_null() {
                libc::free(self.data as *mut c_void);
            }
        }
    }
}

impl Clone for CharContainer {
    fn clone(&self) -> Self {
        CharContainer::new(self.into()).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_from_str() {
        let content = "abcdef";
        let container = CharContainer::new(content).expect("Allocation failed");
        let data: &str = (&container).into();

        assert_eq!(data, content);
        assert_eq!(container.size, content.len());
    }

    #[test]
    fn test_empty() {
        let content = "";
        let container = CharContainer::new(content).expect("Allocation failed");
        let data: &str = (&container).into();

        assert_eq!(data, "");
        assert_eq!(container.size, 0);
        assert!(!container.data.is_null()); // \0
    }

    #[test]
    fn test_clone() {
        let container = CharContainer::new("abcdef").expect("Allocation failed");
        let copy = container.clone();
        let data: &str = (&container).into();
        let data_copy: &str = (&copy).into();

        assert_eq!(data, data_copy);
        assert_ne!(container.data, copy.data);
        assert_eq!(container.size, copy.size);
    }

    #[test]
    fn test_from_c_str() {
        let content = CString::new("abcdef").unwrap();
        let ptr = content.as_ptr();
        unsafe {
            let container = CharContainer::from_c_str(ptr).expect("Allocation failed");
            let data: &str = (&container).into();
            assert_eq!(data, "abcdef");
            assert_eq!(container.size, 6);
        }
    }

    #[test]
    fn test_c_str_stop() {
        let content_bytes = b"abc\0def";
        let ptr = content_bytes.as_ptr() as *const i8;

        unsafe {
            let container = CharContainer::from_c_str(ptr).expect("Allocation failed");
            let data: &str = (&container).into();
            assert_eq!(data, "abc");
            assert_eq!(container.size, 3);
        }
    }

    #[test]
    fn test_internal_terminator() {
        let content = "abc";
        let container = CharContainer::new(content).expect("Allocation failed");

        unsafe {
            assert_eq!(*container.data.add(0), b'a');
            assert_eq!(*container.data.add(1), b'b');
            assert_eq!(*container.data.add(2), b'c');
            assert_eq!(*container.data.add(3), 0); // \0
        }
    }
}
