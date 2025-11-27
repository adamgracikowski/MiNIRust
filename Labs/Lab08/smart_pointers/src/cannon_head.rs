use std::{borrow::Cow, collections::VecDeque};

pub fn canon_head<'a>(xs: &'a VecDeque<i32>) -> Option<Cow<'a, VecDeque<i32>>> {
    match xs.front() {
        None => return Some(Cow::Borrowed(xs)),
        Some(&front) if front % 2 != 0 => return Some(Cow::Borrowed(xs)),
        _ => {}
    };

    let mut owned_xs = xs.clone();
    let len = owned_xs.len();

    for _ in 0..len {
        let front = *owned_xs.front().unwrap();

        if front % 2 != 0 {
            return Some(Cow::Owned(owned_xs));
        }

        let val = owned_xs.pop_front().unwrap();
        owned_xs.push_back(val);
    }

    None
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::canon_head;

    #[test]
    fn test_canon_head() {
        let xs = VecDeque::from(vec![2, 4, 6, 8]);
        let result = canon_head(&xs);
        assert!(result.is_none());

        let xs = VecDeque::from(vec![1, 2, 3, 4]);
        let result = canon_head(&xs).unwrap();
        assert!(matches!(result, std::borrow::Cow::Borrowed(_)));
        assert_eq!(&*result, &xs);

        let xs = VecDeque::from(vec![2, 4, 5, 6]);
        let result = canon_head(&xs).unwrap();
        assert!(matches!(result, std::borrow::Cow::Owned(_)));
        let expected = VecDeque::from(vec![5, 6, 2, 4]);
        assert_eq!(&*result, &expected);
    }
}
