mod constant;
mod expression;
mod var;

use constant::Const;
use expression::E;
use var::Var;

fn main() {
    let c1 = Const::Numeric(42);
    let c2 = Const::Named("a".to_string());

    let vx = Var::X;
    let vy = Var::Y;
    let vz = Var::Z;

    let e_const = E::constant(c1.clone());
    let e_named = E::constant(c2.clone());
    let e_var_x = E::var(vx);
    let e_var_y = E::var(vy);
    let e_func = E::func("sin".to_string(), e_var_y.clone());
    let e_add = E::add(e_const.clone(), e_var_x.clone());
    let e_mul = E::mul(e_add.clone(), e_func.clone());
    let e_neg = E::neg(e_mul.clone());
    let e_inv = E::inv(e_neg.clone());

    let diff_wrt_x = e_mul.clone().diff(vx);
    let diff_wrt_y = e_mul.clone().diff(vy);
    let substituted = e_named
        .clone()
        .substitute("a", E::constant(Const::Numeric(7)));

    let double_inv = E::inv(E::inv(E::var(vz)));
    let uninv_result = double_inv.uninv();

    let double_neg = E::neg(E::neg(E::var(vz)));
    let unneg_result = double_neg.unneg();

    let expressions: Vec<Box<E>> = vec![
        e_const,
        e_named,
        e_var_x,
        e_var_y,
        e_func,
        e_add,
        e_mul,
        e_neg,
        e_inv,
        diff_wrt_x,
        diff_wrt_y,
        substituted,
        uninv_result,
        unneg_result,
        E::var(vz),
        E::constant(Const::Numeric(7)),
        E::func("f".to_string(), E::inv(E::var(vy))),
    ];

    let expr_strings = expressions
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>();
    let max_len = expr_strings
        .iter()
        .map(|s| s.chars().count())
        .max()
        .unwrap_or(0)
        .min(60);

    println!(
        "{:<4} {:<width$} {:>12}",
        "ID",
        "Expression",
        "Arguments",
        width = max_len
    );
    println!("{:-<1$}", "", 4 + 1 + max_len + 1 + 12);

    for (i, (expr, expr_str)) in expressions.iter().zip(expr_strings.iter()).enumerate() {
        let display_str = if expr_str.chars().count() > max_len {
            let mut s = expr_str
                .chars()
                .take(max_len.saturating_sub(3))
                .collect::<String>();
            s.push_str("...");
            s
        } else {
            expr_str.clone()
        };

        println!(
            "{:<4} {display_str:<width$} {:>12}",
            i + 1,
            expr.arg_count(),
            width = max_len
        );
    }

    println!();
    println!("Constants: {c1}, {c2}");
    println!("Variables: {vx}, {vy}, {vz}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const_to_string() {
        let c_num = Const::Numeric(42);
        let c_name = Const::Named("a".into());
        assert_eq!(c_num.to_string(), "42");
        assert_eq!(c_name.to_string(), "a");
    }

    #[test]
    fn test_var_to_string() {
        assert_eq!(Var::X.to_string(), "X");
        assert_eq!(Var::Y.to_string(), "Y");
        assert_eq!(Var::Z.to_string(), "Z");
    }

    #[test]
    fn test_builder_constant_var() {
        let e_const = E::constant(Const::Numeric(5));
        let e_var = E::var(Var::X);
        assert_eq!(e_const.to_string(), "5");
        assert_eq!(e_var.to_string(), "X");
    }

    #[test]
    fn test_builder_add() {
        let expr = E::add(E::constant(Const::Numeric(2)), E::var(Var::X));
        assert_eq!(expr.to_string(), "(2 + X)");
    }

    #[test]
    fn test_builder_neg() {
        let expr = E::neg(E::var(Var::X));
        assert_eq!(expr.to_string(), "-(X)");
    }

    #[test]
    fn test_builder_mul() {
        let expr = E::mul(E::var(Var::X), E::var(Var::Y));
        assert_eq!(expr.to_string(), "(X * Y)");
    }

    #[test]
    fn test_builder_inv() {
        let expr = E::inv(E::var(Var::X));
        assert_eq!(expr.to_string(), "1/(X)");
    }

    #[test]
    fn test_builder_func() {
        let expr = E::func("f".into(), E::var(Var::X));
        assert_eq!(expr.to_string(), "f(X)");
    }

    #[test]
    fn test_expr_to_string_complex() {
        let expr1 = E::add(E::constant(Const::Numeric(2)), E::var(Var::X));
        let expr2 = E::mul(E::neg(E::var(Var::Y)), E::inv(E::var(Var::Z)));
        let complex = E::add(
            E::func("f".into(), expr1.clone()),
            E::func("g".into(), expr2.clone()),
        );
        assert_eq!(complex.to_string(), "(f((2 + X)) + g((-(Y) * 1/(Z))))");
    }

    #[test]
    fn test_diff_add_vars() {
        let expr = E::add(E::var(Var::X), E::var(Var::Y));
        let d = expr.diff(Var::X);
        assert_eq!(d.to_string(), "(1 + 0)");
    }

    #[test]
    fn test_unpack_inv_inv() {
        let double_inv = E::inv(E::inv(E::var(Var::X)));
        let inner = double_inv.clone().unpack_inv_inv().unwrap();
        assert_eq!(inner.to_string(), "X");
    }

    #[test]
    fn test_unpack_neg_neg() {
        let double_neg = E::neg(E::neg(E::neg(E::neg(E::neg(E::var(Var::Y))))));
        let inner = double_neg.clone().unneg();
        assert_eq!(inner.to_string(), "-(Y)");
    }

    #[test]
    fn test_simplify_double_inv() {
        let double_inv = E::inv(E::inv(E::var(Var::X)));
        let simplified = double_inv.uninv();
        assert_eq!(simplified.to_string(), "X");
    }

    #[test]
    fn test_simplify_double_neg() {
        let double_neg = E::neg(E::neg(E::var(Var::X)));
        let simplified = double_neg.unneg();
        assert_eq!(simplified.to_string(), "X");
    }

    #[test]
    fn test_substitute_named_constant() {
        let expr = E::add(E::constant(Const::Named("a".into())), E::var(Var::X));
        let substituted = expr.substitute("a", E::constant(Const::Numeric(10)));
        assert_eq!(substituted.to_string(), "(10 + X)");
    }

    #[test]
    fn test_substitute_deep() {
        let expr = E::mul(
            E::constant(Const::Named("a".into())),
            E::func("f".into(), E::constant(Const::Named("a".into()))),
        );
        let substituted = expr.substitute("a", E::constant(Const::Numeric(3)));
        assert_eq!(substituted.to_string(), "(3 * f(3))");
    }

    #[test]
    fn test_diff_neg() {
        let expr = E::neg(E::var(Var::X));
        let d = expr.diff(Var::X);
        assert_eq!(d.to_string(), "-(1)");
    }

    #[test]
    fn test_diff_mul() {
        let expr = E::mul(E::var(Var::X), E::var(Var::Y));
        let d = expr.diff(Var::X);
        assert_eq!(d.to_string(), "((1 * Y) + (X * 0))");
    }

    #[test]
    fn test_diff_inv() {
        let expr = E::inv(E::var(Var::X));
        let d = expr.diff(Var::X);
        assert_eq!(d.to_string(), "(-(1/((X * X))) * 1)");
    }

    #[test]
    fn test_diff_const_numeric() {
        let expr = E::constant(Const::Numeric(7));
        let d = expr.diff(Var::X);
        assert_eq!(d.to_string(), "0");
    }

    #[test]
    fn test_diff_const_named() {
        let expr = E::constant(Const::Named("a".into()));
        let d = expr.diff(Var::X);
        assert_eq!(d.to_string(), "0");
    }

    #[test]
    fn test_diff_func() {
        let expr = E::func("f".into(), E::var(Var::X));
        let d = expr.diff(Var::X);
        assert_eq!(d.to_string(), "(f_X(X) * 1)");
    }

    #[test]
    fn test_diff_var_same() {
        let d = E::var(Var::X).diff(Var::X);
        assert_eq!(d.to_string(), "1");
    }

    #[test]
    fn test_diff_var_other() {
        let d = E::var(Var::Y).diff(Var::X);
        assert_eq!(d.to_string(), "0");
    }

    #[test]
    fn test_diff_big_expression() {
        // (((X + -(Y)) * 1/(Z)) + (f((X * Y)) + g(1/(X))))
        let part1 = E::add(E::var(Var::X), E::neg(E::var(Var::Y)));
        let part2 = E::inv(E::var(Var::Z));
        let a = E::mul(part1.clone(), part2.clone());
        let xy = E::mul(E::var(Var::X), E::var(Var::Y));
        let b = E::func("f".into(), xy);
        let inv_x = E::inv(E::var(Var::X));
        let c = E::func("g".into(), inv_x);
        let big = E::add(a.clone(), E::add(b.clone(), c.clone()));

        assert_eq!(
            big.to_string(),
            "(((X + -(Y)) * 1/(Z)) + (f((X * Y)) + g(1/(X))))"
        );

        let d = big.diff(Var::X);
        assert_eq!(
            d.to_string(),
            "((((1 + -(0)) * 1/(Z)) + ((X + -(Y)) * (-(1/((Z * Z))) * 0))) + ((f_X((X * Y)) * ((1 * Y) + (X * 0))) + (g_X(1/(X)) * (-(1/((X * X))) * 1))))"
        );
    }

    #[test]
    fn test_arg_count_zeroary() {
        assert_eq!(E::constant(Const::Numeric(1)).arg_count(), 0);
        assert_eq!(E::var(Var::X).arg_count(), 0);
    }

    #[test]
    fn test_arg_count_unary() {
        assert_eq!(E::neg(E::var(Var::X)).arg_count(), 1);
        assert_eq!(E::inv(E::var(Var::X)).arg_count(), 1);
        assert_eq!(E::func("f".into(), E::var(Var::X)).arg_count(), 1);
    }

    #[test]
    fn test_arg_count_binary() {
        assert_eq!(E::add(E::var(Var::X), E::var(Var::Y)).arg_count(), 2);
        assert_eq!(E::mul(E::var(Var::X), E::var(Var::Z)).arg_count(), 2);
    }
}
