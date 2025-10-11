use super::{InPlaceNumericOperations, NumberWithUnits};

pub fn mul_vals_vec(nums: Vec<NumberWithUnits>) -> NumberWithUnits {
    let mut result = NumberWithUnits::unitless(1.0);
    for num in nums {
        result.mul_in_place(&num);
    }
    result
}