use crate::core::{Context, Expr, Stmt};

pub struct Print<T: Expr> {
    inner: T,
}

pub fn print<T: Expr>(inner: T) -> Print<T> {
    Print { inner }
}

impl<T: Expr> Stmt for Print<T> {
    fn exec_stmt(&mut self, context: &Context) {
        let value = self.inner.exec_expr(context);
        println!("{value}");
    }
}

pub struct Nothing;

impl Stmt for Nothing {
    fn exec_stmt(&mut self, _context: &Context) {}
}

pub fn nothing() -> Nothing {
    Nothing
}

pub struct Seq<T: Stmt, U: Stmt> {
    first: T,
    second: U,
}

impl<T: Stmt, U: Stmt> Stmt for Seq<T, U> {
    fn exec_stmt(&mut self, context: &Context) {
        self.first.exec_stmt(context);
        self.second.exec_stmt(context);
    }
}

impl<T: Stmt> Seq<T, Nothing> {
    pub fn shorten_1(self) -> T {
        self.first
    }
}

impl<U: Stmt> Seq<Nothing, U> {
    pub fn shorten_2(self) -> U {
        self.second
    }
}

impl Seq<Nothing, Nothing> {
    pub fn collapse(self) -> Nothing {
        Nothing
    }
}

pub fn seq<T: Stmt, U: Stmt>(first: T, second: U) -> Seq<T, U> {
    Seq { first, second }
}