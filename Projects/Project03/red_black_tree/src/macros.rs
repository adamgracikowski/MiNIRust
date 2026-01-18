#[macro_export]
macro_rules! red_black_tree {
    ( $( $key:expr => $val:expr ),* $(,)? ) => {
        {
            let mut tree = $crate::RedBlackTree::default();
            $(
                if let Some(value) = $crate::CharContainer::new($val) {
                    let _ = tree.insert($key, value);
                }
            )*
            tree
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_macro_usage() {
        let tree = red_black_tree! {
            1 => "a",
            2 => "b",
            3 => "c"
        };

        assert_eq!(tree.get(1), Some("a"));
        assert_eq!(tree.get(2), Some("b"));
        assert_eq!(tree.get(3), Some("c"));
        assert!(!tree.contains(4));
    }
}
