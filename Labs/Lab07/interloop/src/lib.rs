use itertools::Itertools;
use std::collections::HashSet;

pub fn wrap_call(f1: impl Fn(u32) -> u32, f2: impl FnOnce(u32, u32) -> u32) -> u32 {
    let f1_rename = f1;
    f2(f1_rename(1), f1_rename(2))
}

pub fn make_counter(start: i64) -> impl FnMut() -> i64 {
    let mut start = start;
    move || {
        let result = start;
        start += 1;
        result
    }
}

pub fn sum_squares_odd_loop(list: &[u32]) -> u32 {
    let mut sum = 0;
    for el in list {
        if el % 2 == 1 {
            sum += el * el;
        }
    }
    sum
}

pub fn sum_squares_odd(list: &[u32]) -> u32 {
    list.iter()
        .filter(|el| *el % 2 == 1)
        .map(|el| el * el)
        .sum()
}

pub fn vertices_loop(edges: &[(u32, u32)]) -> Vec<u32> {
    let mut hash = HashSet::new();
    let mut result = Vec::new();
    for (u, v) in edges {
        if !hash.contains(v) {
            hash.insert(v);
            result.push(*v);
        }
        if !hash.contains(u) {
            hash.insert(u);
            result.push(*u);
        }
    }
    result.sort();
    result
}

pub fn vertices(edges: &[(u32, u32)]) -> Vec<u32> {
    edges
        .iter()
        .flat_map(|(a, b)| [*a, *b])
        .unique()
        .sorted()
        .collect()
}

pub fn cycles_2_loop(edges: &[(u32, u32)]) -> Vec<u32> {
    let mut hash = HashSet::new();
    let mut result = Vec::new();
    for (a, b) in edges {
        for (c, d) in edges {
            if a == d && b == c && a != b {
                if !hash.contains(a) {
                    hash.insert(a);
                    result.push(*a);
                }
                if !hash.contains(b) {
                    hash.insert(b);
                    result.push(*b);
                }
            }
        }
    }
    result.sort();
    result
}

pub fn cycles_2(edges: &[(u32, u32)]) -> Vec<u32> {
    let cycles = edges
        .iter()
        .cartesian_product(edges.iter())
        .filter(|((a, b), (c, d))| a == d && b == c && a != b)
        .map(|(&(a, b), _)| (a, b))
        .collect::<Vec<_>>();
    vertices(&cycles)
}

pub fn primes_loop(n: u32) -> Vec<u32> {
    let mut result = Vec::new();

    'c: for x in 2..n {
        for p in &result {
            if p * p > x {
                break;
            }
            if x % p == 0 {
                continue 'c;
            }
        }
        result.push(x);
    }

    result
}

pub fn primes(n: u32) -> Vec<u32> {
    (2..n)
        .filter(|&x| !(2..).take_while(|&d| d * d <= x).any(|d| x % d == 0))
        .collect()
}

pub fn run_length_encode_loop(list: &[u32]) -> Vec<(u32, usize)> {
    let mut result = Vec::new();

    if list.is_empty() {
        return result;
    }

    let mut cur = list[0];
    let mut count = 1;

    for &el in &list[1..] {
        if el == cur {
            count += 1;
        } else {
            result.push((cur, count));
            cur = el;
            count = 1;
        }
    }

    result.push((cur, count));
    result
}

pub fn run_length_encode(list: &[u32]) -> Vec<(u32, usize)> {
    list.chunk_by(|a, b| a == b)
        .map(|c| (c[0], c.len()))
        .collect()
}

pub fn compose_all_loop(fns: &[fn(i32) -> i32]) -> impl Fn(i32) -> i32 {
    move |v| {
        let mut u = v;
        for f in fns {
            u = f(u);
        }
        u
    }
}

pub fn compose_all(fns: &[fn(i32) -> i32]) -> impl Fn(i32) -> i32 {
    move |v| fns.iter().fold(v, |acc, f| f(acc))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nasty_test() {
        let f1 = |x| x * 100;
        let mut vec = Vec::new();
        let f2 = move |v1, v2| {
            vec.push(v1 + v2);
            let val = vec[0];
            std::mem::drop(vec);
            val
        };
        let val = super::wrap_call(f1, f2);
        assert_eq!(val, 300);
    }

    #[test]
    fn counter_basic() {
        let mut c = make_counter(10);
        assert_eq!(c(), 10);
        assert_eq!(c(), 11);
        assert_eq!(c(), 12);
        let mut c2 = make_counter(-3);
        assert_eq!(c2(), -3);
        assert_eq!(c2(), -2);
        assert_eq!(c(), 13);
    }

    #[test]
    fn sum_squares_odd_cases() {
        let empty: &[u32] = &[];
        assert_eq!(sum_squares_odd_loop(empty), 0);
        assert_eq!(sum_squares_odd(empty), 0);
        let evens = [2, 4, 6];
        assert_eq!(sum_squares_odd_loop(&evens), 0);
        assert_eq!(sum_squares_odd(&evens), 0);
        let nums = [1, 2, 3, 4, 5];
        assert_eq!(sum_squares_odd_loop(&nums), 35);
        assert_eq!(sum_squares_odd(&nums), 35);
    }

    #[test]
    fn vertices_and_cycles() {
        let edges = [(1, 2), (2, 1), (3, 4), (4, 3), (5, 5), (2, 3)];
        let v_loop = vertices_loop(&edges);
        let v_iter = vertices(&edges);
        assert_eq!(v_loop, v_iter);
        assert_eq!(v_loop, vec![1, 2, 3, 4, 5]);
        let c_loop = cycles_2_loop(&edges);
        let c_iter = cycles_2(&edges);
        assert_eq!(c_loop, c_iter);
        assert_eq!(c_loop, vec![1, 2, 3, 4]);
    }

    #[test]
    fn cycles_2_duplicates() {
        let edges = [(1, 2), (2, 1), (1, 2), (2, 1), (2, 2)];
        assert_eq!(cycles_2_loop(&edges), vec![1, 2]);
        assert_eq!(cycles_2(&edges), vec![1, 2]);
    }

    #[test]
    fn empty_graph() {
        let edges: [(u32, u32); 0] = [];
        assert_eq!(vertices_loop(&edges), Vec::<u32>::new());
        assert_eq!(vertices(&edges), Vec::<u32>::new());
        assert_eq!(cycles_2_loop(&edges), Vec::<u32>::new());
        assert_eq!(cycles_2(&edges), Vec::<u32>::new());
    }

    #[test]
    fn primes_examples() {
        assert_eq!(primes_loop(0), Vec::<u32>::new());
        assert_eq!(primes(0), Vec::<u32>::new());
        assert_eq!(primes_loop(2), Vec::<u32>::new());
        assert_eq!(primes(2), Vec::<u32>::new());
        assert_eq!(primes_loop(3), vec![2]);
        assert_eq!(primes(3), vec![2]);
        assert_eq!(primes_loop(10), vec![2, 3, 5, 7]);
        assert_eq!(primes(10), vec![2, 3, 5, 7]);
        assert_eq!(primes_loop(30), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
        assert_eq!(primes(30), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn primes_large_count() {
        let p100 = primes(100);
        assert_eq!(p100.len(), 25);
        assert_eq!(p100.last(), Some(&97));
        assert_eq!(p100, primes_loop(100));
    }

    #[test]
    fn wrap_call_fn_ptr() {
        fn times2(x: u32) -> u32 {
            x * 2
        }
        let val = wrap_call(times2, |a, b| a + b);
        assert_eq!(val, 6);
    }

    #[test]
    fn rle_basic_and_edges() {
        assert_eq!(run_length_encode_loop(&[]), Vec::<(u32, usize)>::new());
        assert_eq!(run_length_encode(&[]), Vec::<(u32, usize)>::new());
        assert_eq!(run_length_encode_loop(&[7]), vec![(7, 1)]);
        assert_eq!(run_length_encode(&[7]), vec![(7, 1)]);
        let data = [1, 1, 2, 2, 2, 1];
        let expect = vec![(1, 2), (2, 3), (1, 1)];
        assert_eq!(run_length_encode_loop(&data), expect);
        assert_eq!(run_length_encode(&data), expect);
    }

    #[test]
    fn rle_varied_runs() {
        let data = [3, 3, 3, 3, 2, 2, 9, 9, 9, 1, 1, 1, 1, 1];
        let expect = vec![(3, 4), (2, 2), (9, 3), (1, 5)];
        assert_eq!(run_length_encode_loop(&data), expect);
        assert_eq!(run_length_encode(&data), expect);
    }

    #[test]
    fn compose_all_identity_and_order() {
        fn add1(x: i32) -> i32 {
            x + 1
        }
        fn times2(x: i32) -> i32 {
            x * 2
        }
        fn square(x: i32) -> i32 {
            x * x
        }

        let id_iter = compose_all(&[]);
        let id_loop = compose_all_loop(&[]);
        assert_eq!(id_iter(42), 42);
        assert_eq!(id_loop(42), 42);

        let f_iter = compose_all(&[add1, times2, square]);
        let f_loop = compose_all_loop(&[add1, times2, square]);
        assert_eq!(f_iter(3), 64);
        assert_eq!(f_loop(3), 64);

        let g_iter = compose_all(&[square, times2, add1]);
        assert_eq!(g_iter(3), ((3 * 3) * 2) + 1);
    }

    #[test]
    fn compose_all_matches_loop() {
        fn f1(x: i32) -> i32 {
            x - 5
        }
        fn f2(x: i32) -> i32 {
            x * 3
        }
        fn f3(x: i32) -> i32 {
            x + 10
        }
        let funcs = [f1, f2, f3];
        let c1 = compose_all(&funcs);
        let c2 = compose_all_loop(&funcs);
        for x in [-10, -1, 0, 1, 7, 20] {
            assert_eq!(c1(x), c2(x));
        }
    }
}
