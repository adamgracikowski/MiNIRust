mod double_string;
mod number_with_units;

use double_string::DoubleString;
use number_with_units::{mul_vals, mul_vals_vec, NumberWithUnits, NumericOperations};

fn main() {
    let num1 = NumberWithUnits::unitless(12.22);
    let num2 = NumberWithUnits::with_unit(3.25, "km".to_string());
    let num3 = NumberWithUnits::with_unit_from(&num2, 2.71);

    println!("{num1:?}");
    println!("{num2:?}");
    println!("{num3:?}");

    let distance1 = NumberWithUnits::with_unit(5.0, "m".to_string());
    let distance2 = NumberWithUnits::with_unit(3.0, "m".to_string());
    let distance = distance1.add(distance2);
    let time = NumberWithUnits::with_unit(2.0, "s".to_string());
    let speed = NumericOperations::div(distance, time); // distance.div(time) conflicts with std::ops::Div

    println!("{speed:?}");

    let nums = vec![
        NumberWithUnits::with_unit(2.0, "m".to_string()),
        NumberWithUnits::with_unit(3.0, "m".to_string()),
        NumberWithUnits::with_unit(4.0, "m".to_string()),
    ];

    let product = mul_vals(&nums);
    let product_again = mul_vals(&nums);

    println!("Product using slice: {product:?}");
    println!("Product using slice again: {product_again:?}");

    let product_vec = mul_vals_vec(nums.clone());
    let product_vec_again = mul_vals_vec(nums);

    println!("Product using vector: {product_vec:?}");
    println!("Product using vector again: {product_vec_again:?}");

    let string = String::from("Hello");
    let str_slice = "World";

    let double_string1 = DoubleString::from_strs(&string, str_slice);
    let double_string2 = DoubleString::from_strings(&string, &str_slice.to_string());

    double_string1.show();
    double_string2.show();
}
