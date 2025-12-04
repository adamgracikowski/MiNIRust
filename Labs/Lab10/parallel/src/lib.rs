#![allow(dead_code)]

mod shapes;
mod transformations;

pub use shapes::*;
pub use transformations::*;

use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

pub fn spawn_sum(v: Vec<i32>) {
    let handle = thread::spawn(move || v.iter().sum::<i32>());
    println!("{}", handle.join().expect("Failed to receive the sum..."));
}

pub fn sum_scoped(parts: &[&[i32]]) -> i32 {
    thread::scope(|s| {
        let mut handles = Vec::with_capacity(parts.len());
        for part in parts {
            handles.push(s.spawn(move || part.iter().copied().sum::<i32>()));
        }

        let mut total = 0;
        for h in handles {
            total += h.join().expect("Failed to compute partial sum...");
        }

        total
    })
}

pub fn parallel_increment(n_threads: usize, iters: usize) -> i64 {
    let counter = Arc::new(Mutex::new(0_i64));

    let mut handles = Vec::with_capacity(n_threads);

    for i in 0..n_threads {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for j in 0..iters {
                let mut num = counter.lock().unwrap_or_else(|e| e.into_inner());
                if j == iters - 1 && i % 2 == 0 {
                    panic!("Thread {i} panicked on last iteration");
                }
                *num += 1;
            }
        });

        handles.push(handle);
    }

    for (i, handle) in handles.into_iter().enumerate() {
        if handle.join().is_err() {
            print!("Thread {i} panicked!");
        }
    }

    *counter.lock().unwrap_or_else(|e| e.into_inner())
}

pub fn pipeline(n: i32, threads: usize) -> i32 {
    let (tx, rx) = mpsc::channel::<u32>();

    let sum_handle = thread::spawn(move || {
        let mut total = 0;
        for v in rx {
            total += v;
        }
        total
    });

    let mut handles = Vec::with_capacity(threads);

    for _ in 0..threads {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            for i in 1..=n as u32 {
                tx_clone
                    .send(i)
                    .expect("Receiver was dropped while sending");
            }
        });
        handles.push(handle);
    }

    drop(tx);

    for handle in handles {
        handle.join().expect("Sender thread panicked");
    }

    sum_handle.join().expect("Sum thread panicked") as i32
}

#[cfg(test)]
mod tests {
    use std::any::Any;

    use super::*;

    fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() <= eps
    }

    #[test]
    fn test_total_area_generic_rects() {
        let rects = vec![Rect { w: 3.0, h: 4.0 }, Rect { w: 2.5, h: 1.2 }];
        let total = total_area_generic(&rects);
        assert!(approx_eq(total, 15.0, 1e-12));
    }

    #[test]
    fn test_total_area_generic_circles() {
        let circles = vec![Circle { r: 1.0 }, Circle { r: 2.0 }];
        let total = total_area_generic(&circles);
        let expected = std::f64::consts::PI * (1.0 + 4.0);
        assert!(approx_eq(total, expected, 1e-12));
    }

    #[test]
    fn test_total_area_dyn_mixed() {
        let items: Vec<Box<dyn Shape>> = vec![
            Box::new(Rect { w: 3.0, h: 4.0 }),
            Box::new(Circle { r: 1.0 }),
        ];
        let total = total_area_dyn(&items);
        let expected = 12.0 + std::f64::consts::PI * 1.0;
        assert!(approx_eq(total, expected, 1e-12));
    }

    #[test]
    fn test_transform_add_and_apply_all_dyn() {
        // After making Transform object-safe with apply(&self, f64) -> f64
        let mut seq = [1.0, 2.0, -3.0];
        let add = Add { k: 2.0 };
        apply_all_dyn(&mut seq, &add);
        assert!(approx_eq(seq[0], 3.0, 1e-12));
        assert!(approx_eq(seq[1], 4.0, 1e-12));
        assert!(approx_eq(seq[2], -1.0, 1e-12));
        assert_eq!(add.name(), "add");
    }

    #[test]
    fn test_transform_mul_and_apply_all_dyn() {
        let mut seq = [1.5, -2.0, 0.0];
        let mul = Mul { k: -2.0 };
        apply_all_dyn(&mut seq, &mul);
        assert!(approx_eq(seq[0], -3.0, 1e-12));
        assert!(approx_eq(seq[1], 4.0, 1e-12));
        assert!(approx_eq(seq[2], 0.0, 1e-12));
        assert_eq!(mul.name(), "mul");
    }

    #[test]
    fn test_sum_all_i32_mixed_any() {
        let boxes: Vec<Box<dyn Any>> = vec![
            Box::new(5_i32),
            Box::new(String::from("x")),
            Box::new(7_i32),
            Box::new(3_i64),
        ];
        let s = sum_all_i32(&boxes);
        assert_eq!(s, 12);
    }

    #[test]
    fn test_spawn_sum_large() {
        // We cannot easily assert stdout in unit tests without extra crates; just ensure it completes.
        let v: Vec<i32> = (1..=10_000).collect();
        spawn_sum(v);
    }

    #[test]
    fn test_sum_scoped_parts() {
        let a = [1, 2, 3];
        let b = [10];
        let c = [-5, 0];
        let parts: Vec<&[i32]> = vec![&a, &b, &c];
        let s = sum_scoped(&parts);
        assert_eq!(s, 11);
    }

    #[test]
    fn test_parallel_increment_poison_and_total() {
        // Even-index threads panic at last iteration; they miss exactly one increment.
        let total = parallel_increment(4, 5);
        // Expected increments = n_threads * iters - number_of_even_threads
        let expected = (4 * 5 - 2) as i64;
        assert_eq!(total, expected);
    }

    #[test]
    fn test_pipeline_small() {
        let s = pipeline(3, 2);
        assert_eq!(s, 12); // 2 * (1 + 2 + 3)
    }

    #[test]
    fn test_pipeline_medium() {
        let s = pipeline(10, 3);
        assert_eq!(s, 165); // 3 * 55
    }
}
