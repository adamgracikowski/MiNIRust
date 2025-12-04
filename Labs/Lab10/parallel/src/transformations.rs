mod add;
mod mul;

pub use add::Add;
pub use mul::Mul;

use std::any::Any;

/// Starter code version was not 'object safe'
/// due to generic type parameter in apply method
pub trait Transform<T: Copy> {
    fn name(&self) -> &str;
    fn apply(&self, x: T) -> T;
}

pub fn apply_all_dyn(seq: &mut [f64], t: &dyn Transform<f64>) {
    seq.iter_mut().for_each(|item| *item = t.apply(*item));
}

pub fn sum_all_i32(boxes: &[Box<dyn Any>]) -> i32 {
    boxes
        .iter()
        .fold(0_i32, |acc, curr| match curr.downcast_ref() {
            Some(num) => acc + num,
            None => acc,
        })
}
