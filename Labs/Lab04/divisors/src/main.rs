use std::{
    collections::BTreeSet,
    hint::black_box,
    num::NonZeroU32,
    time::{Duration, Instant},
    vec,
};

pub fn divisors(n: NonZeroU32) -> BTreeSet<NonZeroU32> {
    let n = n.get();
    let mut divisors = BTreeSet::new();

    let mut i = 1u32;

    while i * i <= n {
        if n % i == 0 {
            divisors.insert(NonZeroU32::new(i).unwrap());
            let other = n / i;
            if other != i {
                divisors.insert(NonZeroU32::new(other).unwrap());
            }
        }
        i += 1;
    }

    divisors
}

fn benchmark_divisors<F>(f: F)
where
    F: Fn(NonZeroU32) -> BTreeSet<NonZeroU32>,
{
    let mut total_duration = Duration::ZERO;

    for _ in 0..100 {
        let start = Instant::now();

        for n in 1u32..=100 {
            let input = black_box(NonZeroU32::new(n).unwrap());
            let _ = black_box(f(black_box(input)));
        }

        total_duration += start.elapsed();
    }

    let avg = total_duration / 100;
    println!("Average time: {:.6} ms", avg.as_secs_f64() * 1000.0);
}

fn assert_sorted(buf: &[i32]) {
    for window in buf.windows(2) {
        let (first, second) = (window[0], window[1]);
        if first > second {
            panic!("The array is not sorted: {first} > {second}")
        }
    }
}

fn main() {
    // should panic!
    // let unsorted = vec![1, 3, 2];
    // assert_sorted(&unsorted);

    let sorted = vec![1, 2, 3];
    assert_sorted(&sorted);

    let inputs = vec![
        NonZeroU32::new(10).unwrap(),
        NonZeroU32::new(45).unwrap(),
        NonZeroU32::new(64).unwrap(),
        NonZeroU32::new(1024).unwrap(),
    ];

    for input in inputs {
        let result = divisors(input);
        dbg!(result);
    }

    benchmark_divisors(divisors);
}
