use std::fmt::Display;

use super::{Const, Var};

#[derive(Clone, Debug)]
pub enum E {
    Add(Box<E>, Box<E>),
    Neg(Box<E>),
    Mul(Box<E>, Box<E>),
    Inv(Box<E>),
    Const(Const),
    Func { name: String, arg: Box<E> },
    Var(Var),
}

impl E {
    pub fn add(lhs: Box<Self>, rhs: Box<Self>) -> Box<Self> {
        Box::new(Self::Add(lhs, rhs))
    }

    pub fn neg(expr: Box<Self>) -> Box<Self> {
        Box::new(Self::Neg(expr))
    }

    pub fn mul(lhs: Box<Self>, rhs: Box<Self>) -> Box<Self> {
        Box::new(Self::Mul(lhs, rhs))
    }

    pub fn inv(expr: Box<Self>) -> Box<Self> {
        Box::new(Self::Inv(expr))
    }

    pub fn constant(constant: Const) -> Box<Self> {
        Box::new(Self::Const(constant))
    }

    pub fn func(name: String, arg: Box<Self>) -> Box<Self> {
        Box::new(Self::Func { name, arg })
    }

    pub fn var(variable: Var) -> Box<Self> {
        Box::new(Self::Var(variable))
    }

    pub fn arg_count(&self) -> u32 {
        match self {
            Self::Var(_) | Self::Const(_) => 0,
            Self::Neg(_) | Self::Inv(_) | E::Func { .. } => 1,
            Self::Add(_, _) | Self::Mul(_, _) => 2,
        }
    }

    pub fn diff(self, by: Var) -> Box<Self> {
        match self {
            Self::Add(a, b) => Self::add(a.diff(by), b.diff(by)),
            Self::Neg(a) => Self::neg(a.diff(by)),
            Self::Mul(a, b) => Self::add(
                Self::mul(a.clone().diff(by), b.clone()),
                Self::mul(a, b.diff(by)),
            ),
            Self::Inv(a) => Self::mul(
                Self::neg(Self::inv(Self::mul(a.clone(), a.clone()))),
                a.clone().diff(by),
            ),
            Self::Const(_) => Self::constant(Const::Numeric(0)),
            Self::Func { name, arg } => Self::mul(
                Self::func(format!("{name}_{by}"), arg.clone()),
                arg.diff(by),
            ),
            Self::Var(v) => {
                if v == by {
                    Self::constant(Const::Numeric(1))
                } else {
                    Self::constant(Const::Numeric(0))
                }
            }
        }
    }

    pub fn unpack_inv_inv(self) -> Option<Box<Self>> {
        if let Self::Inv(frac) = self {
            if let Self::Inv(frac) = *frac {
                return Some(frac);
            }
        }

        None
    }

    pub fn uninv(self: Box<Self>) -> Box<Self> {
        let mut f = self;

        while let Some(e) = f.clone().unpack_inv_inv() {
            f = e;
        }

        f
    }

    pub fn unpack_neg_neg(self) -> Option<Box<Self>> {
        if let Self::Neg(u) = self
            && let Self::Neg(v) = *u
        {
            return Some(v);
        }

        None
    }

    pub fn unneg(self: Box<Self>) -> Box<Self> {
        let mut f = self;

        while let Some(e) = f.clone().unpack_neg_neg() {
            f = e;
        }

        f
    }
    pub fn substitute(self, name: &str, value: Box<Self>) -> Box<Self> {
        match self {
            Self::Add(lhs, rhs) => Self::add(
                lhs.substitute(name, value.clone()),
                rhs.substitute(name, value),
            ),
            Self::Neg(expr) => Self::neg(expr.substitute(name, value)),
            Self::Mul(lhs, rhs) => Self::mul(
                lhs.substitute(name, value.clone()),
                rhs.substitute(name, value),
            ),
            Self::Inv(expr) => Self::inv(expr.substitute(name, value.clone())),
            Self::Func { name: func, arg } => Self::func(func, arg.substitute(name, value.clone())),
            Self::Const(constant) => match constant {
                Const::Numeric(number) => Self::constant(Const::Numeric(number)),
                Const::Named(constant_name) => {
                    if constant_name == name {
                        return value;
                    }
                    Self::constant(Const::Named(constant_name))
                }
            },
            Self::Var(variable) => Self::var(variable),
        }
    }
}

impl Display for E {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add(a, b) => {
                write!(f, "({a} + {b})")
            }
            Self::Neg(a) => {
                write!(f, "-({a})")
            }
            Self::Mul(a, b) => {
                write!(f, "({a} * {b})")
            }
            Self::Inv(a) => {
                write!(f, "1/({a})")
            }
            Self::Const(c) => {
                write!(f, "{c}")
            }
            Self::Func { name, arg } => {
                write!(f, "{name}({arg})")
            }
            Self::Var(v) => {
                write!(f, "{v}")
            }
        }
    }
}
