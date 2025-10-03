pub fn tuple_operations() {
    let (id, name, factor) = make_tuple();
    println!("Tuple: id={id}, name='{name}', factor={factor}");

    let mut outer_count = 0;

    'outer: loop {
        outer_count += 1;
        for i in 0..5 {
            println!("outer_count={outer_count}, i={i}");
            if outer_count >= 2 && i == 3 {
                println!("Condition met — breaking 'outer loop");
                break 'outer;
            }
        }
    }
}

fn make_tuple() -> (i32, String, f64) {
    (1, "user".to_string(), std::f64::consts::PI)
}
