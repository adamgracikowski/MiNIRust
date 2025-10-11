#[allow(dead_code)]
pub trait NumericOperations {
    fn add(self, other: Self) -> Self;
    fn mul(self, other: Self) -> Self;
    fn div(self, other: Self) -> Self;
}