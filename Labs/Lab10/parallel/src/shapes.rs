mod circle;
mod rect;

pub use circle::Circle;
pub use rect::Rect;

pub trait Shape {
    fn area(&self) -> f64;
}

pub fn total_area_generic<T: Shape>(items: &[T]) -> f64 {
    items.iter().fold(0_f64, |acc, cur| acc + cur.area())
}

pub fn total_area_dyn(items: &[Box<dyn Shape>]) -> f64 {
    items.iter().fold(0_f64, |acc, cur| acc + cur.area())
}
