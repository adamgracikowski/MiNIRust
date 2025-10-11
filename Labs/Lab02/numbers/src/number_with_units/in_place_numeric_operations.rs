#[allow(dead_code)]
pub trait InPlaceNumericOperations {
    fn add_in_place(&mut self, other: &Self);
    fn mul_in_place(&mut self, other: &Self);
    fn div_in_place(&mut self, other: &Self);
}