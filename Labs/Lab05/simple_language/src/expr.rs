use crate::core::{Context, Expr, Stmt};

impl Expr for u64 {
    fn exec_expr(&mut self, _context: &Context) -> u64 {
        *self
    }
}

pub struct When<T: Expr, U: Expr, W: Expr> {
    condition: T,
    if_true: U,
    if_false: W,
}

impl<T: Expr, U: Expr, W: Expr> Expr for When<T, U, W> {
    fn exec_expr(&mut self, context: &Context) -> u64 {
        match self.condition.exec_expr(context) {
            0 => self.if_false.exec_expr(context),
            _ => self.if_true.exec_expr(context),
        }
    }
}

pub fn when<T: Expr, U: Expr, W: Expr>(condition: T, if_true: U, if_false: W) -> When<T, U, W> {
    When {
        condition,
        if_true,
        if_false,
    }
}

pub struct Repeat<const N: u32, T: Stmt> {
    pub inner: T,
}

impl<const N: u32, T: Stmt> Stmt for Repeat<N, T> {
    fn exec_stmt(&mut self, context: &Context) {
        for _ in 1..=N {
            self.inner.exec_stmt(context);
        }
    }
}

pub fn repeat<const N: u32, T: Stmt>(inner: T) -> Repeat<N, T> {
    Repeat { inner }
}

pub struct Constant {
    name: &'static str,
}

impl Expr for Constant {
    fn exec_expr(&mut self, context: &Context) -> u64 {
        *context.get(self.name).unwrap()
    }
}

pub fn constant(name: &'static str) -> Constant {
    Constant { name }
}

pub struct ReadFrom<'a> {
    pub variable: &'a u64,
}

impl<'a> Expr for ReadFrom<'a> {
    fn exec_expr(&mut self, _context: &Context) -> u64 {
        *self.variable
    }
}

pub fn read_from<'a>(variable: &'a u64) -> ReadFrom<'a> {
    ReadFrom { variable }
}

pub struct SaveIn<'a, T: Expr> {
    pub destination: &'a mut u64,
    pub inner: T,
}

impl<'a, T: Expr> Expr for SaveIn<'a, T> {
    fn exec_expr(&mut self, context: &Context) -> u64 {
        let value = self.inner.exec_expr(context);
        *self.destination = value;
        value
    }
}

pub fn save_in<'a, T: Expr>(destination: &'a mut u64, inner: T) -> SaveIn<'a, T> {
    SaveIn { destination, inner }
}

pub struct Volatile<'a, T: Expr> {
    pub destination: &'a mut u64,
    pub name: &'static str,
    pub inner: T,
}

impl<'a, T: Expr> Expr for Volatile<'a, T> {
    fn exec_expr(&mut self, context: &Context) -> u64 {
        let mut volatile_context = context.clone();
        volatile_context.insert(self.name, *self.destination);

        let value = self.inner.exec_expr(&volatile_context);

        *self.destination = value;

        value
    }
}

pub fn volatile<'a, T: Expr>(
    destination: &'a mut u64,
    name: &'static str,
    inner: T,
) -> Volatile<'a, T> {
    Volatile {
        destination,
        name,
        inner,
    }
}